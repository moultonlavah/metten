use super::{
    budget::{Budget, Traffic},
    signing::Key,
    *,
};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Message, client::IntoClientRequest, protocol::WebSocketConfig},
};
type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub struct Connection {
    socket: Option<Socket>,
    provider: Provider,
    fence: Fence,
    generation: u64,
    _memory: tokio::sync::OwnedSemaphorePermit,
}
impl Connection {
    fn fail(&mut self) {
        self.fence.revoke();
        self.socket.take();
    }
    pub async fn close(&mut self, deadline: Instant) -> Result<(), Failure> {
        self.fence.revoke();
        let mut socket = self.socket.take();
        if let Some(socket) = &mut socket {
            tokio::time::timeout_at(deadline, socket.close(None))
                .await
                .map_err(|_| Failure::Deadline)?
                .map_err(|_| Failure::Transport)
        } else {
            Ok(())
        }
    }
}
// A borrowed public operation may be dropped by its caller's select/timeout.
struct InFlight<'a> {
    connection: &'a mut Connection,
    completed: bool,
}
impl Drop for InFlight<'_> {
    fn drop(&mut self) {
        if !self.completed {
            self.connection.fail();
        }
    }
}
pub struct Transport {
    environment: Environment,
    client: reqwest::Client,
    budgets: [Arc<std::sync::Mutex<Budget>>; 3],
    memory: memory::Memory,
    retries: [std::sync::Mutex<budget::Retry>; 3],
    http_slots: Arc<tokio::sync::Semaphore>,
    handshake_slots: [Arc<tokio::sync::Semaphore>; 3],
    #[cfg(test)]
    peer: Option<std::net::SocketAddr>,
}
impl Transport {
    pub fn new(environment: Environment) -> Result<Self, Failure> {
        let client = reqwest::Client::builder()
            .https_only(true)
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .pool_max_idle_per_host(1)
            .http1_only()
            .build()
            .map_err(|_| Failure::Transport)?;
        Ok(Self {
            environment,
            client,
            budgets: std::array::from_fn(|_| Arc::new(std::sync::Mutex::new(Budget::new()))),
            memory: memory::Memory::default(),
            retries: std::array::from_fn(|index| {
                std::sync::Mutex::new(budget::Retry::new(index as u64 + 1))
            }),
            http_slots: Arc::new(tokio::sync::Semaphore::new(2)),
            handshake_slots: std::array::from_fn(|_| Arc::new(tokio::sync::Semaphore::new(1))),
            #[cfg(test)]
            peer: None,
        })
    }
    fn rest_url(&self, path: &str) -> String {
        #[cfg(test)]
        if let Some(peer) = self.peer {
            return format!("http://{peer}{path}");
        }
        format!("{}{path}", self.environment.rest())
    }
    /// Actual transport polls: [REST reads, WebSocket handshakes, control writes].
    pub fn attempts(&self, provider: Provider) -> Result<[u64; 3], Failure> {
        Ok(self.budgets[provider.index()]
            .lock()
            .map_err(|_| Failure::Unavailable)?
            .attempts)
    }
    #[cfg(test)]
    pub fn controlled(peer: std::net::SocketAddr) -> Self {
        let mut transport = Self::new(Environment::Demo).unwrap();
        transport.client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .build()
            .unwrap();
        transport.peer = Some(peer);
        transport
    }
    pub fn memory(&self) -> &memory::Memory {
        &self.memory
    }
    pub fn counters(&self, provider: Provider) -> Result<[u64; 5], Failure> {
        let budget = self.budgets[provider.index()]
            .lock()
            .map_err(|_| Failure::Unavailable)?;
        Ok([
            budget.attempts[0],
            budget.attempts[1],
            budget.attempts[2],
            budget.pongs,
            budget.data_messages,
        ])
    }
    pub fn retry_usable(&self, provider: Provider, now: Instant) -> Result<(), Failure> {
        self.retries[provider.index()]
            .lock()
            .map_err(|_| Failure::Unavailable)?
            .usable(now);
        Ok(())
    }
    pub fn slow_retry(&self, provider: Provider) -> Result<bool, Failure> {
        Ok(self.retries[provider.index()]
            .lock()
            .map_err(|_| Failure::Unavailable)?
            .slow())
    }
    async fn start_attempt(
        &self,
        provider: Provider,
        permit: &mut budget::Permit,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
    ) -> Result<(), Failure> {
        loop {
            if *stop.borrow() {
                return Err(Failure::Stopped);
            }
            let delay = {
                let mut retry = self.retries[provider.index()]
                    .lock()
                    .map_err(|_| Failure::Unavailable)?;
                match retry.claim(Instant::now()) {
                    Some(delay) => Some(delay),
                    None => {
                        permit.start()?;
                        None
                    }
                }
            };
            match delay {
                Some(delay) => wait(stop, deadline, delay).await?,
                None => return Ok(()),
            }
        }
    }
    async fn permit(
        &self,
        provider: Provider,
        traffic: Traffic,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
    ) -> Result<budget::Permit, Failure> {
        loop {
            if *stop.borrow() {
                return Err(Failure::Stopped);
            }
            let result =
                budget::Permit::reserve(self.budgets[provider.index()].clone(), traffic, deadline);
            match result {
                Err(Failure::Budget) => wait(stop, deadline, Duration::from_millis(100)).await?,
                result => return result,
            }
        }
    }
    pub async fn get(
        &self,
        route: &Route,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
        fence: &Fence,
    ) -> Result<memory::Bytes, Failure> {
        let deadline = deadline.min(Instant::now() + Duration::from_secs(10));
        fence
            .run(deadline, self.get_once_owned(route, stop, deadline, fence))
            .await
    }
    pub async fn connect(
        &self,
        provider: Provider,
        credential: Option<(&str, &Key)>,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
        fence: &Fence,
    ) -> Result<Connection, Failure> {
        let deadline = deadline.min(Instant::now() + Duration::from_secs(10));
        let work = async {
            loop {
                match self
                    .connect_once(provider, credential, stop, deadline, fence)
                    .await
                {
                    Ok(connection) => return Ok(connection),
                    Err(error) => match self.retries[provider.index()]
                        .lock()
                        .map_err(|_| Failure::Unavailable)?
                        .delay(error)
                    {
                        Some(_) => continue,
                        None => return Err(error),
                    },
                }
            }
        };
        fence.run(deadline, work).await
    }
    async fn get_once_owned(
        &self,
        route: &Route,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
        fence: &Fence,
    ) -> Result<memory::Bytes, Failure> {
        if matches!(route, Route::Socket) {
            return Err(Failure::InvalidRequest);
        }
        let path = route.path()?;
        let generation = fence.token();
        if !fence.current(generation) {
            return Err(Failure::Stopped);
        }
        let _slot = bounded(stop, deadline, async {
            self.http_slots
                .clone()
                .acquire_owned()
                .await
                .map_err(|_| Failure::Stopped)
        })
        .await?;
        loop {
            if !fence.current(generation) {
                return Err(Failure::Stopped);
            }
            let mut permit = self
                .permit(Provider::Kalshi, Traffic::Read, stop, deadline)
                .await?;
            let charge = self.memory.reserve(MAX_BYTES)?;
            let mut admission_stop = stop.clone();
            let result = bounded(stop, deadline, async {
                self.start_attempt(Provider::Kalshi, &mut permit, &mut admission_stop, deadline)
                    .await?;
                read_body(&self.client, &self.rest_url(&path), charge).await
            })
            .await;
            if !fence.current(generation) {
                return Err(Failure::Stopped);
            }
            match result {
                Ok(bytes) => return Ok(bytes),
                Err(error) => match self.retries[Provider::Kalshi.index()]
                    .lock()
                    .map_err(|_| Failure::Unavailable)?
                    .delay(error)
                {
                    Some(_) => continue,
                    None => return Err(error),
                },
            }
        }
    }

    async fn connect_once(
        &self,
        provider: Provider,
        credential: Option<(&str, &Key)>,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
        fence: &Fence,
    ) -> Result<Connection, Failure> {
        let url = match provider {
            Provider::Kalshi => self.environment.socket(),
            Provider::Coinbase => "wss://ws-feed.exchange.coinbase.com",
            Provider::Kraken => "wss://ws.kraken.com/v2",
        };
        let generation = fence.token();
        if !fence.current(generation) {
            return Err(Failure::Stopped);
        }
        let _slot = bounded(stop, deadline, async {
            self.handshake_slots[provider.index()]
                .clone()
                .acquire_owned()
                .await
                .map_err(|_| Failure::Stopped)
        })
        .await?;
        let mut permit = self
            .permit(provider, Traffic::Handshake, stop, deadline)
            .await?;
        #[cfg(test)]
        let local = self.peer.map(|peer| format!("ws://{peer}/trade-api/ws/v2"));
        #[cfg(test)]
        let url = local.as_deref().unwrap_or(url);
        let mut request = url
            .into_client_request()
            .map_err(|_| Failure::InvalidRequest)?;
        if provider == Provider::Kalshi {
            let (id, key) = credential.ok_or(Failure::Missing)?;
            if !super::super::provider_credential::valid_id(id) {
                return Err(Failure::InvalidCredential);
            }
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| Failure::Signing)?
                .as_millis();
            let timestamp = u64::try_from(timestamp).map_err(|_| Failure::Signing)?;
            for (name, value) in [
                ("KALSHI-ACCESS-KEY", id.to_owned()),
                ("KALSHI-ACCESS-TIMESTAMP", timestamp.to_string()),
                (
                    "KALSHI-ACCESS-SIGNATURE",
                    key.sign(&Route::Socket, timestamp)?,
                ),
            ] {
                let mut value = value
                    .parse::<tokio_tungstenite::tungstenite::http::HeaderValue>()
                    .map_err(|_| Failure::Signing)?;
                value.set_sensitive(true);
                request.headers_mut().insert(name, value);
            }
        } else if credential.is_some() {
            return Err(Failure::InvalidRequest);
        }

        // Frame + fragmented message + write buffer + 1 MiB protocol/read headroom.
        let memory = self.memory.reserve(4 * MAX_BYTES)?;
        let mut admission_stop = stop.clone();
        let (socket, _) = bounded(stop, deadline, async {
            self.start_attempt(provider, &mut permit, &mut admission_stop, deadline)
                .await?;
            tokio_tungstenite::connect_async_with_config(request, Some(config()), false)
                .await
                .map_err(websocket_failure)
        })
        .await?;
        if !fence.current(generation) {
            return Err(Failure::Stopped);
        }
        Ok(Connection {
            socket: Some(socket),
            provider,
            fence: fence.clone(),
            generation,
            _memory: memory,
        })
    }
}
pub fn config() -> WebSocketConfig {
    WebSocketConfig::default()
        .max_message_size(Some(MAX_BYTES))
        .max_frame_size(Some(MAX_BYTES))
        .write_buffer_size(0)
        .max_write_buffer_size(MAX_BYTES + 1024)
}
async fn read_body(
    client: &reqwest::Client,
    url: &str,
    charge: tokio::sync::OwnedSemaphorePermit,
) -> Result<memory::Bytes, Failure> {
    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(|_| Failure::Transport)?;
    if response.status().is_redirection() {
        return Err(Failure::Protocol);
    }
    if response.status().as_u16() == 401 || response.status().as_u16() == 403 {
        return Err(Failure::Denied);
    }
    if !response.status().is_success() {
        return Err(
            if response.status().is_server_error()
                || matches!(response.status().as_u16(), 408 | 429)
            {
                Failure::Transport
            } else {
                Failure::Protocol
            },
        );
    }
    if response
        .content_length()
        .is_some_and(|n| n > MAX_BYTES as u64)
    {
        return Err(Failure::Capacity);
    }
    let mut bytes = Vec::with_capacity(MAX_BYTES);
    while let Some(chunk) = response.chunk().await.map_err(|_| Failure::Transport)? {
        if chunk.len() > MAX_BYTES - bytes.len() {
            return Err(Failure::Capacity);
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(memory::Bytes {
        data: bytes,
        _charge: charge,
    })
}
// Controlled peers use the same framing/body path, never a runtime-selected host.
#[cfg(test)]
pub async fn test_body(url: &str) -> Result<memory::Bytes, Failure> {
    let client = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .build()
        .unwrap();
    read_body(&client, url, memory::Memory::default().reserve(MAX_BYTES)?).await
}
impl Transport {
    pub async fn send(
        &self,
        connection: &mut Connection,
        command: &Control,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
    ) -> Result<(), Failure> {
        let text = command.encode(connection.provider)?;
        if !connection.fence.current(connection.generation) {
            connection.fail();
            return Err(Failure::Stopped);
        }
        let mut operation = InFlight {
            connection,
            completed: false,
        };
        let connection = &mut *operation.connection;
        let fence = connection.fence.clone();
        let result = fence
            .run(deadline, async {
                let mut permit = self
                    .permit(connection.provider, Traffic::Control, stop, deadline)
                    .await?;
                let socket = connection.socket.as_mut().ok_or(Failure::Stopped)?;
                send_control(socket, text, &mut permit, stop, deadline).await
            })
            .await;
        operation.completed = result.is_ok();
        result
    }
    pub async fn receive(
        &self,
        connection: &mut Connection,
        stop: &mut watch::Receiver<bool>,
        deadline: Instant,
    ) -> Result<memory::Bytes, Failure> {
        let charge = match self.memory.reserve(MAX_BYTES) {
            Ok(charge) => charge,
            Err(error) => {
                connection.fail();
                return Err(error);
            }
        };
        let mut operation = InFlight {
            connection,
            completed: false,
        };
        let connection = &mut *operation.connection;
        let fence = connection.fence.clone();
        let result = fence
            .run(deadline, async {
                loop {
                    if !connection.fence.current(connection.generation) {
                        return Err(Failure::Stopped);
                    }
                    let socket = connection.socket.as_mut().ok_or(Failure::Stopped)?;
                    let message = bounded(stop, deadline, async {
                        socket
                            .next()
                            .await
                            .ok_or(Failure::Transport)?
                            .map_err(|_| Failure::Protocol)
                    })
                    .await?;
                    match message {
                        Message::Text(text) => {
                            let mut budget = self.budgets[connection.provider.index()]
                                .lock()
                                .map_err(|_| Failure::Unavailable)?;
                            budget.data_messages = budget
                                .data_messages
                                .checked_add(1)
                                .ok_or(Failure::Capacity)?;
                            return Ok(memory::Bytes {
                                data: text.as_bytes().to_vec(),
                                _charge: charge,
                            });
                        }
                        Message::Ping(_) => {
                            let mut permit = self
                                .permit(connection.provider, Traffic::KeepAlive, stop, deadline)
                                .await?;
                            bounded(stop, deadline, async {
                                permit.start()?;
                                socket.flush().await.map_err(|_| Failure::Transport)
                            })
                            .await?;
                        }
                        Message::Pong(_) => (),
                        _ => return Err(Failure::Protocol),
                    }
                }
            })
            .await;
        operation.completed = result.is_ok();
        result
    }
}

#[cfg(test)]
pub async fn local_connection(
    address: std::net::SocketAddr,
    provider: Provider,
    fence: Fence,
) -> Connection {
    let (socket, _) = tokio_tungstenite::connect_async_with_config(
        format!("ws://{address}"),
        Some(config()),
        false,
    )
    .await
    .unwrap();
    let generation = fence.token();
    Connection {
        socket: Some(socket),
        provider,
        fence,
        generation,
        _memory: memory::Memory::default().reserve(4 * MAX_BYTES).unwrap(),
    }
}

fn websocket_failure(error: tokio_tungstenite::tungstenite::Error) -> Failure {
    use tokio_tungstenite::tungstenite::Error;
    match error {
        Error::Http(response) if matches!(response.status().as_u16(), 401 | 403) => Failure::Denied,
        Error::Http(response)
            if response.status().is_server_error()
                || matches!(response.status().as_u16(), 408 | 429) =>
        {
            Failure::Transport
        }
        Error::Io(_) => Failure::Transport,
        _ => Failure::Protocol,
    }
}

pub(super) async fn send_control<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin>(
    socket: &mut WebSocketStream<S>,
    text: String,
    permit: &mut budget::Permit,
    stop: &mut watch::Receiver<bool>,
    deadline: Instant,
) -> Result<(), Failure> {
    bounded(stop, deadline, async {
        permit.start()?;
        socket
            .send(Message::Text(text.into()))
            .await
            .map_err(|_| Failure::Transport)
    })
    .await
}
