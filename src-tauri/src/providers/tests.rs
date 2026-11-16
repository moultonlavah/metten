use super::*;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::pkcs8::EncodePrivateKey;
use rsa::pkcs8::LineEnding;
use rsa::{pkcs1::EncodeRsaPrivateKey, signature::Verifier};
fn ed_pem() -> String {
    ed25519_dalek::SigningKey::from_bytes(&[7; 32])
        .to_pkcs8_pem(LineEnding::LF)
        .unwrap()
        .to_string()
}
#[test]
fn signatures_use_canonical_finite_requests() {
    let message = signing::presign(
        &Route::Markets {
            min_close_ts: 1,
            max_close_ts: 901,
            cursor: None,
        },
        123,
    )
    .unwrap();
    assert_eq!(message, "123GET/trade-api/v2/markets");
    let ed = signing::Key::parse(&ed_pem()).unwrap();
    let route = Route::Socket;
    let sig = STANDARD.decode(ed.sign(&route, 123).unwrap()).unwrap();
    ed25519_dalek::SigningKey::from_bytes(&[7; 32])
        .verifying_key()
        .verify_strict(
            b"123GET/trade-api/ws/v2",
            &ed25519_dalek::Signature::from_slice(&sig).unwrap(),
        )
        .unwrap();
    let rsa = rsa::RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 2048).unwrap();
    for pem in [
        rsa.to_pkcs8_pem(LineEnding::LF).unwrap(),
        rsa.to_pkcs1_pem(LineEnding::LF).unwrap(),
    ] {
        let key = signing::Key::parse(&pem).unwrap();
        assert_eq!(key.kind(), "rsa");
        let sig = STANDARD.decode(key.sign(&route, 123).unwrap()).unwrap();
        rsa::pss::VerifyingKey::<sha2::Sha256>::new(rsa.to_public_key())
            .verify(
                b"123GET/trade-api/ws/v2",
                &rsa::pss::Signature::try_from(sig.as_slice()).unwrap(),
            )
            .unwrap();
    }
    for bad in [
        "garbage".into(),
        "x".repeat(16385),
        rsa::RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 1024)
            .unwrap()
            .to_pkcs8_pem(LineEnding::LF)
            .unwrap()
            .to_string(),
    ] {
        assert!(signing::Key::parse(&bad).is_err());
    }
}
#[test]
fn routes_cannot_escape_manifest() {
    assert_eq!(
        Route::Series.path().unwrap(),
        "/trade-api/v2/series/KXBTC15M"
    );
    for bad in ["../../orders", "KXBTC15M-a?x=y", "KXBTC15M-a/b", "OTHER"] {
        assert!(Route::Market(bad.into()).path().is_err());
    }
    for route in [
        Route::Markets {
            min_close_ts: 1,
            max_close_ts: 4000,
            cursor: None,
        },
        Route::Markets {
            min_close_ts: 1,
            max_close_ts: 2,
            cursor: Some("?orders".into()),
        },
    ] {
        assert!(route.path().is_err());
    }
    assert_eq!(Environment::Demo.account(), "kalshi:demo");
    assert_ne!(
        Environment::Demo.account(),
        Environment::Production.account()
    );
}
#[tokio::test(start_paused = true)]
async fn budgets_count_attempts_and_expire_before_spending() {
    use budget::{Budget, Traffic};
    let mut budget = Budget::new();
    let deadline = Instant::now() + Duration::from_secs(10);
    for _ in 0..5 {
        budget.attempt(Traffic::Read, deadline).unwrap();
    }
    assert_eq!(
        budget.attempt(Traffic::Read, deadline),
        Err(Failure::Budget)
    );
    assert_eq!(budget.attempts, [5, 0, 0]);
    tokio::time::advance(Duration::from_secs(1)).await;
    budget.attempt(Traffic::Read, deadline).unwrap();
    tokio::time::advance(Duration::from_secs(9)).await;
    assert_eq!(
        budget.attempt(Traffic::Read, deadline),
        Err(Failure::Deadline)
    );
    assert_eq!(budget.attempts, [6, 0, 0]);
}
#[tokio::test]
async fn full_intake_revokes_without_waiting_and_obsolete_results_stay_revoked() {
    let (tx, mut rx) = intake();
    let fence = Fence::new();
    admit(
        &tx,
        &fence,
        Intake::new(
            Provider::Kalshi,
            &fence,
            memory::Memory::default().copy(&[1]).unwrap(),
            Instant::now(),
        ),
    )
    .unwrap();
    assert_eq!(
        admit(
            &tx,
            &fence,
            Intake::new(
                Provider::Kalshi,
                &fence,
                memory::Memory::default().copy(&[2]).unwrap(),
                Instant::now()
            )
        ),
        Err(Failure::Capacity)
    );
    assert!(!fence.current(rx.recv().await.unwrap().generation));
    let (stop, mut cancel) = watch::channel(false);
    stop.send_replace(true);
    assert_eq!(
        bounded(
            &mut cancel,
            Instant::now() + SHUTDOWN,
            std::future::pending::<Result<(), Failure>>()
        )
        .await,
        Err(Failure::Stopped)
    );
}
#[tokio::test]
async fn shutdown_retains_nonresponsive_worker_then_explicitly_recovers() {
    let mut owner = Coordinator::new();
    let (finish, wait) = std::sync::mpsc::channel();
    owner.worker = Some(std::thread::spawn(move || {
        wait.recv().unwrap();
    }));
    let old = owner.fence(Provider::Kalshi);
    let token = old.token();
    assert_eq!(
        owner
            .shutdown(Instant::now() + Duration::from_millis(10))
            .await,
        Err(Failure::StillStopping)
    );
    assert!(!old.current(token));
    assert!(owner.worker.is_some());
    assert_eq!(owner.recover(false), Err(Failure::StillStopping));
    finish.send(()).unwrap();
    owner.shutdown(Instant::now() + SHUTDOWN).await.unwrap();
    assert!(owner.finished());
    owner.recover(false).unwrap();
    assert!(!old.current(token));
    assert!(owner.fence(Provider::Kalshi).current(1));
}
#[tokio::test]
async fn owned_async_exit_is_observed_and_deadline_cancels_work() {
    let mut owner = Coordinator::new();
    let mut cancel = owner.cancellation(Provider::Kalshi);
    owner
        .spawn(Provider::Kalshi, async move {
            let _ = cancel.changed().await;
        })
        .unwrap();
    owner.shutdown(Instant::now() + SHUTDOWN).await.unwrap();
    assert!(owner.tasks.is_empty());
    let (_, mut stop) = watch::channel(false);
    assert_eq!(
        bounded(
            &mut stop,
            Instant::now() + Duration::from_millis(1),
            std::future::pending::<Result<(), Failure>>()
        )
        .await,
        Err(Failure::Stopped)
    );
}
#[tokio::test]
async fn controlled_http_rejects_redirect_and_large_body() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    for response in [
        "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:1/secret\r\nContent-Length: 0\r\n\r\n"
            .to_string(),
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            MAX_BYTES + 1
        ),
    ] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = [0; 4096];
            let n = socket.read(&mut request).await.unwrap();
            assert!(
                std::str::from_utf8(&request[..n])
                    .unwrap()
                    .starts_with("GET / HTTP/1.1")
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        });
        assert!(
            transport::test_body(&format!("http://{address}/"))
                .await
                .is_err()
        );
        task.await.unwrap();
    }
}
#[tokio::test]
async fn controlled_websocket_enforces_frame_limit() {
    use futures_util::{SinkExt, StreamExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
        let _ = socket
            .send(tokio_tungstenite::tungstenite::Message::Text(
                "x".repeat(MAX_BYTES + 1).into(),
            ))
            .await;
    });
    let (mut socket, _) = tokio_tungstenite::connect_async_with_config(
        format!("ws://{address}"),
        Some(transport::config()),
        false,
    )
    .await
    .unwrap();
    assert!(socket.next().await.unwrap().is_err());
    drop(socket);
    peer.await.unwrap();
}
#[test]
fn credential_records_validate_before_mutation_and_readback_is_required() {
    use crate::provider_credential::{Credentials, Mutation, Status, Store};
    #[derive(Default)]
    struct Memory {
        items: [Option<Vec<u8>>; 2],
        fail: bool,
        ambiguous: bool,
    }
    impl Store for Memory {
        fn read(&mut self, e: Environment) -> Result<Option<Vec<u8>>, Failure> {
            Ok(self.items[usize::from(e == Environment::Production)].clone())
        }
        fn mutate(&mut self, e: Environment, _: Mutation, b: Option<&[u8]>) -> Result<(), Failure> {
            if self.fail {
                return Err(Failure::Unavailable);
            }
            if !self.ambiguous {
                self.items[usize::from(e == Environment::Production)] = b.map(Vec::from);
            }
            Ok(())
        }
    }
    let mut owner = Coordinator::new();
    owner.revoke();
    let mut credentials = Credentials::new(Memory::default());
    let env = Environment::Demo;
    let pem = ed_pem();
    assert_eq!(
        credentials.change(env, Mutation::Save, Some(("id", "bad")), &mut owner),
        Err(Failure::InvalidCredential)
    );
    assert_eq!(credentials.status(env), Ok(Status::Missing));
    credentials
        .change(env, Mutation::Save, Some(("id", &pem)), &mut owner)
        .unwrap();
    assert_eq!(
        credentials.change(env, Mutation::Save, Some(("other", &pem)), &mut owner),
        Err(Failure::AlreadySaved)
    );
    credentials
        .change(env, Mutation::Replace, Some(("other", &pem)), &mut owner)
        .unwrap();
    assert_eq!(
        credentials.status(env),
        Ok(Status::Saved {
            key_id: "other".into(),
            key_type: "ed25519"
        })
    );
    assert_eq!(
        credentials.status(Environment::Production),
        Ok(Status::Missing)
    );
    credentials
        .change(env, Mutation::Remove, None, &mut owner)
        .unwrap();
    assert_eq!(
        credentials.change(env, Mutation::Remove, None, &mut owner),
        Err(Failure::Missing)
    );
    let mut failed = Credentials::new(Memory {
        fail: true,
        ..Memory::default()
    });
    assert_eq!(
        failed.change(env, Mutation::Save, Some(("id", &pem)), &mut owner),
        Err(Failure::Unavailable)
    );
    let mut ambiguous = Credentials::new(Memory {
        ambiguous: true,
        ..Memory::default()
    });
    assert_eq!(
        ambiguous.change(env, Mutation::Save, Some(("id", &pem)), &mut owner),
        Err(Failure::Ambiguous)
    );
}

#[tokio::test(start_paused = true)]
async fn retry_is_bounded_and_stop_interrupts_backoff() {
    let mut retry = budget::Retry::new(7);
    for seconds in [5, 10, 20, 40, 60] {
        let delay = retry.delay(Failure::Transport).unwrap();
        if seconds == 60 {
            assert_eq!(delay, Duration::from_secs(60));
        } else {
            assert!(delay > Duration::from_secs(seconds));
        }
        assert!(delay <= Duration::from_secs(seconds) + Duration::from_millis(250));
    }
    assert!(retry.slow());
    retry.usable(Instant::now());
    tokio::time::advance(Duration::from_secs(29)).await;
    retry.usable(Instant::now());
    assert!(retry.slow());
    tokio::time::advance(Duration::from_secs(1)).await;
    retry.usable(Instant::now());
    assert!(!retry.slow());
    assert!(budget::Retry::new(1).delay(Failure::Denied).is_none());
    let (stop, mut cancel) = watch::channel(false);
    let task = tokio::spawn(async move {
        bounded(
            &mut cancel,
            Instant::now() + Duration::from_secs(30),
            async {
                tokio::time::sleep(Duration::from_secs(20)).await;
                Ok(())
            },
        )
        .await
    });
    tokio::task::yield_now().await;
    stop.send_replace(true);
    assert_eq!(task.await.unwrap(), Err(Failure::Stopped));
}
#[test]
fn controls_have_exact_fixed_provider_scope() {
    let coinbase = Control::Coinbase { subscribe: true }
        .encode(Provider::Coinbase)
        .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&coinbase).unwrap(),
        serde_json::json!({"type":"subscribe","product_ids":["BTC-USD"],"channels":["matches","heartbeat"]})
    );
    assert!(
        Control::Coinbase { subscribe: true }
            .encode(Provider::Kalshi)
            .is_err()
    );
    assert!(
        Control::KalshiSubscribe {
            id: 1,
            channel: Channel::Book,
            markets: vec!["OTHER".into()]
        }
        .encode(Provider::Kalshi)
        .is_err()
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            &Control::KalshiIndexes { id: 2, sid: 1 }
                .encode(Provider::Kalshi)
                .unwrap()
        )
        .unwrap(),
        serde_json::json!({"id":2,"cmd":"update_subscription","params":{"sid":1,"action":"indexlist"}})
    );
    assert!(
        Control::KalshiSubscribe {
            id: 1,
            channel: Channel::Lifecycle,
            markets: vec!["KXBTC15M-A".into()]
        }
        .encode(Provider::Kalshi)
        .is_err()
    );
}
#[tokio::test]
async fn worker_exit_and_queue_are_owned_until_confirmed() {
    let mut owner = Coordinator::new();
    let (tx, rx) = intake();
    let (seen, received) = std::sync::mpsc::channel();
    owner
        .start_worker(rx, move |item| {
            seen.send(item.bytes.as_slice().to_vec()).unwrap();
        })
        .unwrap();
    let fence = owner.fence(Provider::Coinbase);
    admit(
        &tx,
        &fence,
        Intake::new(
            Provider::Coinbase,
            &fence,
            memory::Memory::default().copy(&[42]).unwrap(),
            Instant::now(),
        ),
    )
    .unwrap();
    // Avoid blocking the runtime while the native worker is scheduled.
    let output = tokio::task::spawn_blocking(move || received.recv_timeout(Duration::from_secs(1)))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(output, vec![42]);
    owner.shutdown(Instant::now() + SHUTDOWN).await.unwrap();
    assert!(owner.worker.is_none());
}

#[tokio::test]
async fn exact_outgoing_control_and_nonresponsive_peer_cancel_close_transport() {
    use futures_util::StreamExt;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
        let text = socket.next().await.unwrap().unwrap().into_text().unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&text).unwrap(),
            serde_json::json!({"type":"subscribe","product_ids":["BTC-USD"],"channels":["matches","heartbeat"]})
        );
        assert!(
            socket
                .next()
                .await
                .is_none_or(|m| m.is_err() || m.unwrap().is_close())
        );
    });
    let fence = Fence::new();
    let generation = fence.token();
    let mut connection =
        transport::local_connection(address, Provider::Coinbase, fence.clone()).await;
    let transport = transport::Transport::new(Environment::Demo).unwrap();
    let (_signal, mut stop) = watch::channel(false);
    transport
        .send(
            &mut connection,
            &Control::Coinbase { subscribe: true },
            &mut stop,
            Instant::now() + SHUTDOWN,
        )
        .await
        .unwrap();
    assert_eq!(
        transport
            .receive(
                &mut connection,
                &mut stop,
                Instant::now() + Duration::from_millis(10)
            )
            .await
            .map(|_| ()),
        Err(Failure::Deadline)
    );
    assert!(!fence.current(generation));
    assert_eq!(
        transport
            .send(
                &mut connection,
                &Control::Coinbase { subscribe: true },
                &mut stop,
                Instant::now() + SHUTDOWN
            )
            .await,
        Err(Failure::Stopped)
    );
    peer.await.unwrap();
}

#[test]
fn canceled_reservations_release_capacity_without_counting_an_attempt() {
    let budget = Arc::new(std::sync::Mutex::new(budget::Budget::new()));
    let deadline = Instant::now() + SHUTDOWN;
    let reserved =
        budget::Permit::reserve(budget.clone(), budget::Traffic::Read, deadline).unwrap();
    assert_eq!(budget.lock().unwrap().attempts, [0, 0, 0]);
    drop(reserved);
    for _ in 0..5 {
        let mut permit =
            budget::Permit::reserve(budget.clone(), budget::Traffic::Read, deadline).unwrap();
        permit.start().unwrap();
    }
    assert_eq!(budget.lock().unwrap().attempts, [5, 0, 0]);
    assert!(matches!(
        budget::Permit::reserve(budget, budget::Traffic::Read, deadline),
        Err(Failure::Budget)
    ));
}

#[tokio::test(start_paused = true)]
async fn expired_work_is_never_polled_and_late_success_is_not_current() {
    let (_signal, mut stop) = watch::channel(false);
    let called = std::sync::atomic::AtomicBool::new(false);
    assert_eq!(
        bounded(&mut stop, Instant::now(), async {
            called.store(true, Ordering::SeqCst);
            Ok(())
        })
        .await,
        Err(Failure::Deadline)
    );
    assert!(!called.load(Ordering::SeqCst));
}

#[test]
fn adapter_memory_is_charged_before_growth_and_follows_retained_payloads() {
    let memory = memory::Memory::default();
    let held = memory.reserve(memory::ADAPTER_BYTES - 3).unwrap();
    let bytes = memory.copy(&[1, 2, 3]).unwrap();
    assert_eq!(memory.used(), memory::ADAPTER_BYTES);
    assert!(matches!(memory.copy(&[4]), Err(Failure::Capacity)));
    drop(bytes);
    assert_eq!(memory.used(), memory::ADAPTER_BYTES - 3);
    drop(held);
    assert_eq!(memory.used(), 0);
}
#[tokio::test]
async fn worker_rejects_old_buffering_and_accepts_a_new_explicit_generation() {
    let mut owner = Coordinator::new();
    let (tx, rx) = intake();
    let (seen, mut received) = tokio::sync::mpsc::channel(1);
    owner
        .start_worker(rx, move |item| {
            seen.try_send(item.bytes.as_slice().to_vec()).unwrap();
        })
        .unwrap();
    let old = owner.fence(Provider::Coinbase);
    admit(
        &tx,
        &old,
        Intake::new(
            Provider::Coinbase,
            &old,
            memory::Memory::default().copy(&[1]).unwrap(),
            Instant::now() - MAX_INTAKE_AGE - Duration::from_secs(1),
        ),
    )
    .unwrap();
    tokio::time::timeout(SHUTDOWN, async {
        while old.current(old.token()) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(received.try_recv().is_err());
    owner.recover_provider(Provider::Coinbase, false).unwrap();
    let fresh = owner.fence(Provider::Coinbase);
    admit(
        &tx,
        &fresh,
        Intake::new(
            Provider::Coinbase,
            &fresh,
            memory::Memory::default().copy(&[2]).unwrap(),
            Instant::now(),
        ),
    )
    .unwrap();
    assert_eq!(
        tokio::time::timeout(SHUTDOWN, received.recv())
            .await
            .unwrap(),
        Some(vec![2])
    );
    owner.shutdown(Instant::now() + SHUTDOWN).await.unwrap();
}
#[tokio::test(start_paused = true)]
async fn revocation_interrupts_permit_wait_without_queue_capacity() {
    let fence = Fence::new();
    let task_fence = fence.clone();
    let work = tokio::spawn(async move {
        task_fence
            .run(
                Instant::now() + SHUTDOWN,
                std::future::pending::<Result<(), Failure>>(),
            )
            .await
    });
    tokio::task::yield_now().await;
    fence.revoke();
    assert_eq!(work.await.unwrap(), Err(Failure::Stopped));
}
#[tokio::test]
async fn partial_frame_timeout_drops_socket_and_never_returns_partial_data() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
        socket.get_mut().write_all(&[0x81, 5, b'a']).await.unwrap();
        let mut b = [0; 1];
        assert_eq!(socket.get_mut().read(&mut b).await.unwrap(), 0);
    });
    let fence = Fence::new();
    let mut connection =
        transport::local_connection(address, Provider::Kraken, fence.clone()).await;
    let transport = transport::Transport::new(Environment::Demo).unwrap();
    let (_signal, mut stop) = watch::channel(false);
    assert!(matches!(
        transport
            .receive(
                &mut connection,
                &mut stop,
                Instant::now() + Duration::from_millis(30)
            )
            .await,
        Err(Failure::Deadline)
    ));
    assert!(!fence.current(fence.token()));
    peer.await.unwrap();
}

#[tokio::test]
async fn outgoing_get_ledger_counts_transient_retries_and_refuses_permanent_errors() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    for transient in [false, true] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let peer = tokio::spawn(async move {
            for attempt in 0..if transient { 2 } else { 1 } {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = [0; 4096];
                let n = socket.read(&mut request).await.unwrap();
                assert!(
                    std::str::from_utf8(&request[..n])
                        .unwrap()
                        .starts_with("GET /trade-api/v2/exchange/status HTTP/1.1\r\n")
                );
                let status = if transient && attempt == 0 {
                    "503 Service Unavailable"
                } else if transient {
                    "200 OK"
                } else {
                    "404 Not Found"
                };
                socket.write_all(format!("HTTP/1.1 {status}\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}").as_bytes()).await.unwrap();
            }
        });
        let transport = transport::Transport::controlled(address);
        let (_signal, mut stop) = watch::channel(false);
        let fence = Fence::new();
        let result = transport
            .get(
                &Route::ExchangeStatus,
                &mut stop,
                Instant::now() + Duration::from_secs(9),
                &fence,
            )
            .await;
        if transient {
            assert_eq!(result.unwrap().as_slice(), b"{}");
        } else {
            assert!(matches!(result, Err(Failure::Protocol)));
        }
        assert_eq!(
            transport.attempts(Provider::Kalshi).unwrap(),
            [if transient { 2 } else { 1 }, 0, 0]
        );
        peer.await.unwrap();
    }
}
#[tokio::test]
async fn denied_handshake_is_one_attempt_and_invalid_credential_is_zero() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        let n = socket.read(&mut request).await.unwrap();
        let request = std::str::from_utf8(&request[..n]).unwrap().to_lowercase();
        assert!(request.starts_with("get /trade-api/ws/v2 http/1.1"));
        assert!(request.contains("kalshi-access-key: controlled"));
        socket
            .write_all(
                b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .await
            .unwrap();
    });
    let transport = transport::Transport::controlled(address);
    let (_signal, mut stop) = watch::channel(false);
    let fence = Fence::new();
    assert!(matches!(
        transport
            .connect(
                Provider::Kalshi,
                None,
                &mut stop,
                Instant::now() + SHUTDOWN,
                &fence
            )
            .await,
        Err(Failure::Missing)
    ));
    assert_eq!(transport.attempts(Provider::Kalshi).unwrap(), [0, 0, 0]);
    let key = signing::Key::parse(&ed_pem()).unwrap();
    assert!(matches!(
        transport
            .connect(
                Provider::Kalshi,
                Some(("controlled", &key)),
                &mut stop,
                Instant::now() + SHUTDOWN,
                &fence
            )
            .await,
        Err(Failure::Denied)
    ));
    assert_eq!(transport.attempts(Provider::Kalshi).unwrap(), [0, 1, 0]);
    peer.await.unwrap();
}

#[tokio::test]
async fn partial_control_send_is_canceled_once_and_connection_is_discarded() {
    use tokio::io::AsyncReadExt;
    let (client, mut peer) = tokio::io::duplex(8);
    let mut socket = tokio_tungstenite::WebSocketStream::from_raw_socket(
        client,
        tokio_tungstenite::tungstenite::protocol::Role::Client,
        Some(transport::config()),
    )
    .await;
    let budget = Arc::new(std::sync::Mutex::new(budget::Budget::new()));
    let deadline = Instant::now() + SHUTDOWN;
    let mut permit =
        budget::Permit::reserve(budget.clone(), budget::Traffic::Control, deadline).unwrap();
    let (signal, mut stop) = watch::channel(false);
    let peer_task = tokio::spawn(async move {
        let mut first = [0; 1];
        peer.read_exact(&mut first).await.unwrap();
        signal.send_replace(true);
        let mut remainder = Vec::new();
        peer.read_to_end(&mut remainder).await.unwrap();
    });
    let command = Control::Coinbase { subscribe: true }
        .encode(Provider::Coinbase)
        .unwrap();
    assert_eq!(
        transport::send_control(&mut socket, command, &mut permit, &mut stop, deadline).await,
        Err(Failure::Stopped)
    );
    drop(socket);
    peer_task.await.unwrap();
    assert_eq!(budget.lock().unwrap().attempts, [0, 0, 1]);
}
#[tokio::test]
async fn body_stream_deadline_releases_owned_memory_and_connection() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        assert!(socket.read(&mut request).await.unwrap() > 0);
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 6\r\n\r\nabc")
            .await
            .unwrap();
        let mut b = [0; 1];
        assert_eq!(socket.read(&mut b).await.unwrap(), 0);
    });
    let transport = transport::Transport::controlled(address);
    let (_signal, mut stop) = watch::channel(false);
    let fence = Fence::new();
    assert!(matches!(
        transport
            .get(
                &Route::ExchangeStatus,
                &mut stop,
                Instant::now() + Duration::from_millis(30),
                &fence
            )
            .await,
        Err(Failure::Deadline)
    ));
    assert_eq!(transport.attempts(Provider::Kalshi).unwrap(), [1, 0, 0]);
    assert_eq!(transport.memory().used(), 0);
    peer.await.unwrap();
}

#[tokio::test]
async fn dropping_public_receive_after_a_partial_frame_revokes_and_closes() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
        socket.get_mut().write_all(&[0x81, 5, b'a']).await.unwrap();
        let mut byte = [0; 1];
        assert_eq!(socket.get_mut().read(&mut byte).await.unwrap(), 0);
    });
    let fence = Fence::new();
    let mut connection =
        transport::local_connection(address, Provider::Kraken, fence.clone()).await;
    let transport = transport::Transport::new(Environment::Demo).unwrap();
    let (_signal, mut stop) = watch::channel(false);
    assert!(
        tokio::time::timeout(
            Duration::from_millis(30),
            transport.receive(&mut connection, &mut stop, Instant::now() + SHUTDOWN)
        )
        .await
        .is_err()
    );
    assert!(!fence.current(fence.token()));
    assert!(matches!(
        transport
            .receive(&mut connection, &mut stop, Instant::now() + SHUTDOWN)
            .await,
        Err(Failure::Stopped)
    ));
    peer.await.unwrap();
}
#[tokio::test]
async fn early_provider_return_and_panic_revoke_queued_evidence() {
    for panic in [false, true] {
        let mut owner = Coordinator::new();
        let fence = owner.fence(Provider::Kalshi);
        owner
            .spawn(Provider::Kalshi, async move {
                if panic {
                    panic!("controlled provider failure");
                }
            })
            .unwrap();
        tokio::task::yield_now().await;
        assert!(!fence.current(fence.token()));
        let (tx, _rx) = intake();
        assert_eq!(
            admit(
                &tx,
                &fence,
                Intake::new(
                    Provider::Kalshi,
                    &fence,
                    memory::Memory::default().copy(&[1]).unwrap(),
                    Instant::now()
                )
            ),
            Err(Failure::Stopped)
        );
        owner.shutdown(Instant::now() + SHUTDOWN).await.unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn aborted_nonresponsive_async_task_retains_ownership_until_actual_exit() {
    let mut owner = Coordinator::new();
    let (started, ready) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::channel();
    owner
        .spawn(Provider::Kalshi, async move {
            started.send(()).unwrap();
            wait.recv().unwrap();
        })
        .unwrap();
    ready.await.unwrap();
    assert_eq!(
        owner
            .stop_provider(Provider::Kalshi, Instant::now() + Duration::from_millis(10))
            .await,
        Err(Failure::StillStopping)
    );
    assert_eq!(
        owner.recover_provider(Provider::Kalshi, false),
        Err(Failure::StillStopping)
    );
    assert!(!owner.finished());
    release.send(()).unwrap();
    owner.shutdown(Instant::now() + SHUTDOWN).await.unwrap();
    owner.recover(false).unwrap();
}
#[tokio::test]
async fn worker_panic_revokes_all_current_generations_and_requires_joined_recovery() {
    let mut owner = Coordinator::new();
    let (tx, rx) = intake();
    let kalshi = owner.fence(Provider::Kalshi);
    let spot = owner.fence(Provider::Coinbase);
    owner
        .start_worker(rx, |_| panic!("controlled worker failure"))
        .unwrap();
    admit(
        &tx,
        &spot,
        Intake::new(
            Provider::Coinbase,
            &spot,
            memory::Memory::default().copy(&[1]).unwrap(),
            Instant::now(),
        ),
    )
    .unwrap();
    tokio::time::timeout(SHUTDOWN, async {
        while spot.current(spot.token()) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(!kalshi.current(kalshi.token()));
    assert_eq!(
        owner.recover_provider(Provider::Kalshi, false),
        Err(Failure::StillStopping)
    );
    assert_eq!(
        owner.shutdown(Instant::now() + SHUTDOWN).await,
        Err(Failure::Unavailable)
    );
    owner.shutdown(Instant::now() + SHUTDOWN).await.unwrap();
    owner.recover(false).unwrap();
}

#[tokio::test(start_paused = true)]
async fn waiting_reservations_cannot_accumulate_a_later_burst() {
    let budget = Arc::new(std::sync::Mutex::new(budget::Budget::new()));
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut held = Vec::new();
    for _ in 0..5 {
        held.push(
            budget::Permit::reserve(budget.clone(), budget::Traffic::Read, deadline).unwrap(),
        );
    }
    tokio::time::advance(Duration::from_secs(2)).await;
    assert!(matches!(
        budget::Permit::reserve(budget.clone(), budget::Traffic::Read, deadline),
        Err(Failure::Budget)
    ));
    for permit in &mut held {
        permit.start().unwrap();
    }
    assert!(matches!(
        budget::Permit::reserve(budget.clone(), budget::Traffic::Read, deadline),
        Err(Failure::Budget)
    ));
}

#[tokio::test]
async fn dropping_public_send_during_budget_wait_discards_the_connection() {
    use futures_util::StreamExt;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
        assert!(socket.next().await.unwrap().unwrap().is_text());
        assert!(socket.next().await.is_none_or(|result| result.is_err()));
    });
    let fence = Fence::new();
    let mut connection =
        transport::local_connection(address, Provider::Coinbase, fence.clone()).await;
    let transport = transport::Transport::new(Environment::Demo).unwrap();
    let (_signal, mut stop) = watch::channel(false);
    transport
        .send(
            &mut connection,
            &Control::Coinbase { subscribe: true },
            &mut stop,
            Instant::now() + SHUTDOWN,
        )
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(
            Duration::from_millis(10),
            transport.send(
                &mut connection,
                &Control::Coinbase { subscribe: false },
                &mut stop,
                Instant::now() + SHUTDOWN
            )
        )
        .await
        .is_err()
    );
    assert!(!fence.current(fence.token()));
    assert_eq!(transport.attempts(Provider::Coinbase).unwrap(), [0, 0, 1]);
    peer.await.unwrap();
}

#[tokio::test]
async fn protocol_pong_is_counted_without_spending_subscription_capacity() {
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::Message;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
        socket.send(Message::Ping(vec![1].into())).await.unwrap();
        assert!(socket.next().await.unwrap().unwrap().is_pong());
        socket.send(Message::Text("{}".into())).await.unwrap();
        assert!(socket.next().await.unwrap().unwrap().is_text());
    });
    let fence = Fence::new();
    let mut connection = transport::local_connection(address, Provider::Coinbase, fence).await;
    let transport = transport::Transport::new(Environment::Demo).unwrap();
    let (_signal, mut stop) = watch::channel(false);
    assert_eq!(
        transport
            .receive(&mut connection, &mut stop, Instant::now() + SHUTDOWN)
            .await
            .unwrap()
            .as_slice(),
        b"{}"
    );
    transport
        .send(
            &mut connection,
            &Control::Coinbase { subscribe: true },
            &mut stop,
            Instant::now() + Duration::from_millis(200),
        )
        .await
        .unwrap();
    assert_eq!(
        transport.counters(Provider::Coinbase).unwrap(),
        [0, 0, 1, 1, 1]
    );
    peer.await.unwrap();
}
#[tokio::test]
async fn metadata_client_never_has_more_than_two_requests_in_flight() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let transport = Arc::new(transport::Transport::controlled(address));
    let mut clients = Vec::new();
    for _ in 0..3 {
        let transport = transport.clone();
        clients.push(tokio::spawn(async move {
            let (_signal, mut stop) = watch::channel(false);
            transport
                .get(
                    &Route::ExchangeStatus,
                    &mut stop,
                    Instant::now() + SHUTDOWN,
                    &Fence::new(),
                )
                .await
        }));
    }
    let mut sockets = Vec::new();
    for _ in 0..2 {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = [0; 4096];
        assert!(socket.read(&mut request).await.unwrap() > 0);
        sockets.push(socket);
    }
    assert!(
        tokio::time::timeout(Duration::from_millis(25), listener.accept())
            .await
            .is_err()
    );
    let mut first = sockets.remove(0);
    first
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}")
        .await
        .unwrap();
    drop(first);
    let (mut third, _) = listener.accept().await.unwrap();
    let mut request = [0; 4096];
    assert!(third.read(&mut request).await.unwrap() > 0);
    sockets.push(third);
    for mut socket in sockets {
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}")
            .await
            .unwrap();
    }
    for client in clients {
        assert_eq!(client.await.unwrap().unwrap().as_slice(), b"{}");
    }
    assert_eq!(transport.attempts(Provider::Kalshi).unwrap(), [3, 0, 0]);
}
#[tokio::test]
async fn obsolete_admission_has_a_bounded_reason_counter() {
    let fence = Fence::new();
    let item = Intake::new(
        Provider::Kalshi,
        &fence,
        memory::Memory::default().copy(&[1]).unwrap(),
        Instant::now(),
    );
    let (tx, _rx) = intake();
    fence.revoke();
    assert_eq!(admit(&tx, &fence, item), Err(Failure::Stopped));
    assert_eq!(fence.rejected(), 1);
    assert_eq!(
        fence.run(Instant::now() + SHUTDOWN, async { Ok(()) }).await,
        Err(Failure::Stopped)
    );
    assert_eq!(fence.rejected(), 2);
}

#[tokio::test(start_paused = true)]
async fn slow_retry_claims_a_minute_slot_even_when_the_attempt_succeeds() {
    let mut retry = budget::Retry::new(1);
    for _ in 0..5 {
        retry.delay(Failure::Transport).unwrap();
    }
    tokio::time::advance(Duration::from_secs(60)).await;
    assert!(retry.claim(Instant::now()).is_none());
    assert_eq!(retry.claim(Instant::now()), Some(Duration::from_secs(60)));
    retry.usable(Instant::now());
    tokio::time::advance(Duration::from_secs(30)).await;
    retry.usable(Instant::now());
    assert!(!retry.slow());
    assert!(retry.claim(Instant::now()).is_none());
    assert!(retry.claim(Instant::now()).is_none());
}

#[tokio::test]
async fn queued_handshake_rechecks_backoff_after_the_first_failure() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let transport = Arc::new(transport::Transport::controlled(address));
    let fence = Fence::new();
    let (_signal, stop) = watch::channel(false);
    let mut clients = Vec::new();
    for _ in 0..2 {
        let transport = transport.clone();
        let fence = fence.clone();
        let mut stop = stop.clone();
        clients.push(tokio::spawn(async move {
            transport
                .connect(
                    Provider::Coinbase,
                    None,
                    &mut stop,
                    Instant::now() + SHUTDOWN,
                    &fence,
                )
                .await
                .map(|_| ())
        }));
    }
    let (mut socket, _) = listener.accept().await.unwrap();
    let mut request = [0; 4096];
    assert!(socket.read(&mut request).await.unwrap() > 0);
    tokio::task::yield_now().await;
    socket
        .write_all(
            b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        )
        .await
        .unwrap();
    drop(socket);
    assert!(
        tokio::time::timeout(Duration::from_millis(100), listener.accept())
            .await
            .is_err()
    );
    fence.revoke();
    for client in clients {
        assert!(matches!(client.await.unwrap(), Err(Failure::Stopped)));
    }
    assert_eq!(transport.attempts(Provider::Coinbase).unwrap(), [0, 1, 0]);
}

#[tokio::test]
async fn established_socket_loss_paces_reconnect_and_restarts_usable_window() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        for _ in 0..6 {
            let (socket, _) = listener.accept().await.unwrap();
            drop(tokio_tungstenite::accept_async(socket).await.unwrap());
        }
    });
    let transport = transport::Transport::controlled(address);
    let (_signal, mut stop) = watch::channel(false);
    let fence = Fence::new();
    let mut connection = transport
        .connect(
            Provider::Coinbase,
            None,
            &mut stop,
            Instant::now() + SHUTDOWN,
            &fence,
        )
        .await
        .unwrap();
    assert_eq!(
        transport
            .receive(&mut connection, &mut stop, Instant::now() + SHUTDOWN)
            .await
            .map(|_| ()),
        Err(Failure::Transport)
    );
    assert!(!fence.current(fence.token()));
    assert!(matches!(
        transport
            .connect(
                Provider::Coinbase,
                None,
                &mut stop,
                Instant::now() + Duration::from_millis(30),
                &Fence::new()
            )
            .await,
        Err(Failure::Deadline)
    ));
    assert_eq!(transport.attempts(Provider::Coinbase).unwrap(), [0, 1, 0]);
    // Direct fixture sockets exercise each failure without waiting through real backoff.
    for failure in 2..=6 {
        let fence = Fence::new();
        let mut connection = transport::local_connection(address, Provider::Coinbase, fence).await;
        if failure == 6 {
            transport
                .retry_usable(Provider::Coinbase, Instant::now())
                .unwrap();
            tokio::time::pause();
            tokio::time::advance(Duration::from_secs(20)).await;
            tokio::time::resume();
        }
        assert_eq!(
            transport
                .receive(&mut connection, &mut stop, Instant::now() + SHUTDOWN)
                .await
                .map(|_| ()),
            Err(Failure::Transport)
        );
        assert_eq!(
            transport.slow_retry(Provider::Coinbase).unwrap(),
            failure >= 5
        );
    }
    peer.await.unwrap();
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(10)).await;
    transport
        .retry_usable(Provider::Coinbase, Instant::now())
        .unwrap();
    assert!(transport.slow_retry(Provider::Coinbase).unwrap());
    tokio::time::advance(Duration::from_secs(29)).await;
    transport
        .retry_usable(Provider::Coinbase, Instant::now())
        .unwrap();
    assert!(transport.slow_retry(Provider::Coinbase).unwrap());
    tokio::time::advance(Duration::from_secs(1)).await;
    transport
        .retry_usable(Provider::Coinbase, Instant::now())
        .unwrap();
    assert!(!transport.slow_retry(Provider::Coinbase).unwrap());
}
