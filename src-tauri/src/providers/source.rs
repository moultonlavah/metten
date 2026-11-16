//! The sole native source writer and its owned provider service; no financial run.
use super::*;
use std::collections::BTreeMap;

pub(super) const SOURCE_STREAMS: [metten_engine::sources::Stream; 6] = [
    metten_engine::sources::Stream::Book,
    metten_engine::sources::Stream::Trades,
    metten_engine::sources::Stream::Brti,
    metten_engine::sources::Stream::Metadata,
    metten_engine::sources::Stream::Coinbase,
    metten_engine::sources::Stream::Kraken,
];
const MARKET_STREAMS: [metten_engine::sources::Stream; 3] = [
    metten_engine::sources::Stream::Book,
    metten_engine::sources::Stream::Trades,
    metten_engine::sources::Stream::Metadata,
];
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct ScheduleLoss {
    pub epoch: Option<i64>,
    pub reason: metten_engine::sources::Unavailable,
    pub terminal: bool,
}
/// Runs only on the existing owned native worker. It owns no financial run.
pub struct SourceWorker {
    pub(super) state: metten_engine::sources::SourceState,
    kalshi: kalshi::Kalshi,
    coinbase: coinbase::Coinbase,
    kraken: kraken::Kraken,
    pub(super) memory: memory::Memory,
    tickers: Vec<String>,
    requirements: Vec<metten_engine::strategy::SourceRequirement>,
    details: [Option<wire::Details>; 6],
    market_details: BTreeMap<String, [Option<wire::Details>; 2]>,
    schedule: watch::Sender<Option<ScheduleLoss>>,
    worker_signal: Option<watch::Sender<bool>>,
    number: u64,
    at: u64,
    epoch: Instant,
    epoch_utc_ms: i64,
    links: [Option<(Fence, u64)>; 3],
    pub(super) controls: [Option<mpsc::Sender<Control>>; 3],
    command_id: u32,
    gates: [Arc<AtomicU64>; 6],
    book_sid: Option<u32>,
    trade_sid: Option<u32>,
    progress: Option<watch::Sender<transport::Progress>>,
    blocked: [bool; 6],
    index_requested: bool,
    #[cfg(test)]
    pub(super) lease: Option<u64>,
    _base_charge: Arc<tokio::sync::OwnedSemaphorePermit>,
}
impl SourceWorker {
    pub fn new(
        state: metten_engine::sources::SourceState,
        memory: memory::Memory,
    ) -> Result<Self, Failure> {
        let base_charge = memory.reserve(64 * 1024)?;
        let _initial_view_charge = memory.reserve(256 * 1024)?;
        let view = state.snapshot(0).map_err(|_| Failure::InvalidRequest)?;
        let mut requirements = Vec::new();
        for b in &view.bindings {
            if !requirements
                .iter()
                .any(|r: &metten_engine::strategy::SourceRequirement| {
                    r.input_id == b.requirement.input_id
                })
            {
                requirements.push(b.requirement.clone())
            }
        }
        let tickers = view
            .markets
            .iter()
            .map(|m| m.descriptor.market_id.clone())
            .collect();
        Ok(Self {
            state,
            details: std::array::from_fn(|_| None),
            market_details: BTreeMap::new(),
            schedule: watch::channel(None).0,
            _base_charge: Arc::new(base_charge),
            worker_signal: None,
            kalshi: kalshi::Kalshi::new(memory.clone()),
            coinbase: coinbase::Coinbase::default(),
            kraken: kraken::Kraken::default(),
            memory,
            tickers,
            requirements,
            number: 0,
            at: 0,
            epoch: Instant::now(),
            epoch_utc_ms: utc_now_ms()?,
            links: std::array::from_fn(|_| None),
            controls: std::array::from_fn(|_| None),
            command_id: 100,
            gates: std::array::from_fn(|_| Arc::new(AtomicU64::new(1))),
            book_sid: None,
            trade_sid: None,
            progress: None,
            blocked: [false; 6],
            index_requested: false,
            #[cfg(test)]
            lease: None,
        })
    }
    pub fn tokens(&self) -> [metten_engine::sources::StreamToken; 6] {
        SOURCE_STREAMS.map(|s| self.state.token(s))
    }
    pub fn snapshot(&self, at: u64) -> Result<SourceView, Failure> {
        if self.schedule.borrow().is_some() {
            return Err(Failure::Stopped);
        }
        if self
            .gates
            .iter()
            .any(|g| g.load(Ordering::SeqCst) == u64::MAX)
        {
            return Err(Failure::Capacity);
        }
        let charge = self.memory.reserve(256 * 1024)?;
        Ok(SourceView {
            requirements: self.requirements.clone(),
            blocked: self.blocked,
            details: self.details.clone(),
            market_details: self.market_details.clone(),
            schedule: self.schedule.clone(),
            epoch_utc_ms: self.epoch_utc_ms,
            worker_signal: self.worker_signal.clone(),
            snapshot: self
                .state
                .snapshot(at)
                .map_err(|_| Failure::InvalidRequest)?,
            fences: self.links.clone(),
            epochs: self.gates.each_ref().map(|g| g.load(Ordering::SeqCst)),
            gates: self.gates.clone(),
            _charge: charge,
        })
    }
    pub fn connection(
        &mut self,
        provider: Provider,
        token: metten_engine::sources::StreamToken,
        state: metten_engine::sources::Connection,
        at: u64,
    ) -> Result<metten_engine::sources::SourceEffect, Failure> {
        let stream = match provider {
            Provider::Kalshi => metten_engine::sources::Stream::Book,
            Provider::Coinbase => metten_engine::sources::Stream::Coinbase,
            Provider::Kraken => metten_engine::sources::Stream::Kraken,
        };
        if token != self.state.token(stream) {
            return Err(Failure::Stopped);
        }
        if state != metten_engine::sources::Connection::Open {
            revoke_gates(&self.gates, provider_streams(provider))?;
            if let Some((fence, _)) = &self.links[provider.index()] {
                fence.revoke();
            }
            self.links[provider.index()] = None;
            match provider {
                Provider::Kalshi => {
                    self.book_sid = None;
                    self.trade_sid = None;
                    self.progress = None;
                    self.index_requested = false;
                    self.kalshi.reconnect();
                }
                Provider::Coinbase => self.coinbase.reconnect(),
                Provider::Kraken => self.kraken.reconnect(),
            }
        }
        let provider = engine_provider(provider);
        self.state
            .apply(source_envelope(
                at,
                metten_engine::sources::SourceEvent::Connection {
                    provider,
                    token,
                    state,
                },
            ))
            .map_err(|_| Failure::Protocol)
    }
    pub fn process(
        &mut self,
        item: Intake,
        at: u64,
        completed_frontier_ms: u64,
        mut consume: impl FnMut(metten_engine::sources::SourceEffect) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        use metten_engine::sources::{Scope, Unavailable};
        if !self.memory.owns(&item.bytes) {
            return Err(Failure::InvalidRequest);
        }
        let tokens = item.source_tokens.ok_or(Failure::InvalidRequest)?;
        if !item.fence.current(item.generation) {
            item.fence.reject();
            return Err(Failure::Stopped);
        }
        if at < self.at {
            return Err(Failure::InvalidRequest);
        }
        self.at = at;
        let expired = Instant::now().saturating_duration_since(item.received_at) > MAX_INTAKE_AGE;
        let wire_stream = if item.provider == Provider::Kalshi && !expired {
            match self.kalshi.stream(item.bytes.as_slice()) {
                Ok(stream) => stream,
                Err(error) => {
                    item.fence.fail(error);
                    revoke_gates(&self.gates, provider_streams(item.provider))?;
                    consume(
                        self.state
                            .invalidate(
                                Scope::Provider(engine_provider(item.provider)),
                                Unavailable::InvalidValue,
                                at,
                                None,
                            )
                            .map_err(|_| Failure::Protocol)?,
                    )?;
                    return Err(error);
                }
            }
        } else {
            None
        };
        if let Some(stream) = wire_stream
            && (self.blocked[stream as usize]
                || tokens[stream as usize] != self.state.token(stream))
        {
            self.kalshi.skip_obsolete(item.bytes.as_slice())?;
            if stream == metten_engine::sources::Stream::Book
                && let (Some(progress), Some(sid)) = (&item.progress, self.book_sid)
            {
                let token = self.state.token(stream);
                let needs_retry = {
                    let p = progress.borrow();
                    p.tokens[stream as usize] == Some(token) && p.recovery[0].is_none()
                };
                if needs_retry && !self.tickers.is_empty() {
                    // A receive begun before recovery cannot certify its first returned frame; retry under the new receive identity.
                    self.command_id = self.command_id.checked_add(1).ok_or(Failure::Capacity)?;
                    progress.send_modify(|p| {
                        p.recover(Control::KalshiBookUpdate {
                            id: self.command_id,
                            sid,
                            action: BookAction::Snapshot,
                            markets: self.tickers.clone(),
                            token: Some(token),
                        })
                    });
                }
            }
            item.fence.reject();
            return Err(Failure::Stopped);
        }
        self.links[item.provider.index()] = Some((item.fence.clone(), item.generation));
        if item.provider == Provider::Kalshi {
            self.progress = item.progress.clone();
            if let Some(progress) = &self.progress {
                progress.send_modify(|p| {
                    for stream in SOURCE_STREAMS {
                        if self.blocked[stream as usize] {
                            p.disable(stream);
                        }
                    }
                });
            }
        }
        let _scratch = if expired {
            None
        } else {
            Some(match self.memory.reserve(4 * MAX_BYTES) {
                Ok(charge) => charge,
                Err(error) => {
                    item.fence.revoke();
                    consume(
                        self.state
                            .invalidate(
                                Scope::Provider(engine_provider(item.provider)),
                                Unavailable::Overflow,
                                at,
                                None,
                            )
                            .map_err(|_| Failure::Unavailable)?,
                    )?;
                    return Err(error);
                }
            })
        };
        let receipt = u64::try_from(
            item.received_at
                .saturating_duration_since(self.epoch)
                .as_millis(),
        )
        .map_err(|_| Failure::InvalidRequest)?;
        #[cfg(test)]
        let receipt = item.receipt_ms.unwrap_or(receipt);
        let clock = metten_engine::sources::Clock {
            at_ms: at,
            receipt_ms: receipt,
            received_through_ms: at,
            completed_frontier_ms,
        };
        let state = &mut self.state;
        let number = &mut self.number;
        #[cfg(test)]
        let lease = self.lease;
        #[cfg(not(test))]
        let lease = None;
        let outbound = self.controls[item.provider.index()].clone();
        let book_sid = &mut self.book_sid;
        let trade_sid = &mut self.trade_sid;
        let command_id = &mut self.command_id;
        let tickers = &self.tickers;
        let gates = &self.gates;
        let details = &mut self.details;
        let market_details = &mut self.market_details;
        let index_requested = &mut self.index_requested;
        let blocked = &mut self.blocked;
        let schedule = self.schedule.clone();
        let mut emit = |event| {
            if !item.fence.current(item.generation) {
                return Err(Failure::Stopped);
            }
            if let wire::Event::Prices(p) = &event
                && MARKET_STREAMS.contains(&p.stream)
                && schedule.borrow().is_some()
            {
                return Ok(());
            }
            if let wire::Event::Gap(stream, sid) = event {
                if blocked[stream as usize] {
                    return Ok(());
                }
                let scopes = if stream == metten_engine::sources::Stream::Metadata {
                    &MARKET_STREAMS[..]
                } else {
                    std::slice::from_ref(&stream)
                };
                revoke_gates(gates, scopes)?;
                consume(
                    state
                        .invalidate(
                            metten_engine::sources::Scope::Stream(stream),
                            metten_engine::sources::Unavailable::Gap,
                            at,
                            None,
                        )
                        .map_err(|_| Failure::Protocol)?,
                )?;
                if stream == metten_engine::sources::Stream::Book {
                    let token = state.token(stream);
                    if let Some(progress) = &item.progress {
                        progress.send_modify(|p| p.desired_book = Some(token));
                    }
                    if let Some(outbound) = &outbound {
                        *command_id = command_id.checked_add(1).ok_or(Failure::Capacity)?;
                        let control = Control::KalshiBookUpdate {
                            id: *command_id,
                            sid,
                            action: BookAction::Snapshot,
                            markets: tickers.clone(),
                            token: Some(state.token(stream)),
                        };
                        if let Some(progress) = &item.progress {
                            progress.send_modify(|p| p.recover(control.clone()));
                        } else {
                            outbound.try_send(control).map_err(|_| Failure::Capacity)?;
                        }
                    }
                }
                if let Some(outbound) = &outbound {
                    *command_id = command_id.checked_add(1).ok_or(Failure::Capacity)?;
                    let token = state.token(stream);
                    let control = match stream {
                        metten_engine::sources::Stream::Brti => Some(Control::KalshiIndexes {
                            id: *command_id,
                            sid,
                            token: Some(token),
                        }),
                        metten_engine::sources::Stream::Trades if !tickers.is_empty() => {
                            Some(Control::KalshiTradeUpdate {
                                id: *command_id,
                                sid,
                                action: BookAction::Add,
                                markets: tickers.clone(),
                                token,
                            })
                        }
                        _ => None,
                    };
                    if let Some(control) = control {
                        if let Some(progress) = &item.progress {
                            progress.send_modify(|p| p.recover(control.clone()));
                        } else {
                            outbound.try_send(control).map_err(|_| Failure::Capacity)?;
                        }
                    }
                }
                return Ok(());
            }
            if let wire::Event::ControlRejected(id) = event {
                let progress = item.progress.as_ref().ok_or(Failure::Protocol)?;
                let stream = { progress.borrow().requested_stream(id) }.ok_or(Failure::Protocol)?;
                if blocked[stream as usize] {
                    return Ok(());
                }
                let scopes = if stream == metten_engine::sources::Stream::Metadata {
                    &MARKET_STREAMS[..]
                } else {
                    std::slice::from_ref(&stream)
                };
                revoke_gates(gates, scopes)?;
                for scope in scopes {
                    blocked[*scope as usize] = true;
                }
                consume(
                    state
                        .invalidate(
                            metten_engine::sources::Scope::Stream(stream),
                            metten_engine::sources::Unavailable::InvalidValue,
                            at,
                            None,
                        )
                        .map_err(|_| Failure::Protocol)?,
                )?;
                progress.send_modify(|p| {
                    for scope in scopes {
                        p.disable(*scope);
                    }
                });
                return Ok(());
            }
            if let wire::Event::ControlAcknowledged(id) = event {
                if let Some(progress) = &item.progress {
                    progress.send_modify(|p| p.acknowledge(id));
                }
                return Ok(());
            }
            if let wire::Event::Acknowledged(stream, sid) = &event {
                if blocked[*stream as usize] {
                    return Ok(());
                }
                match stream {
                    metten_engine::sources::Stream::Book => *book_sid = Some(*sid),
                    metten_engine::sources::Stream::Trades => *trade_sid = Some(*sid),
                    metten_engine::sources::Stream::Brti if !*index_requested => {
                        if let Some(outbound) = &outbound {
                            *command_id = command_id.checked_add(1).ok_or(Failure::Capacity)?;
                            outbound
                                .try_send(Control::KalshiIndexes {
                                    id: *command_id,
                                    sid: *sid,
                                    token: None,
                                })
                                .map_err(|_| Failure::Capacity)?;
                            *index_requested = true;
                        }
                    }
                    _ => (),
                }
            }
            match &event {
                wire::Event::Absent(stream) => {
                    revoke_gates(gates, std::slice::from_ref(stream))?;
                    if let Some(progress) = &item.progress {
                        progress.send_modify(|p| p.received(*stream, None));
                    }
                }
                wire::Event::MetadataChanged => revoke_gates(gates, &MARKET_STREAMS)?,
                wire::Event::Prices(p) if p.readings.iter().any(|r| r.value.is_none()) => {
                    revoke_gates(gates, std::slice::from_ref(&p.stream))?
                }
                _ => (),
            }
            let acknowledged = if let wire::Event::Acknowledged(stream, _) = &event {
                Some(*stream)
            } else {
                None
            };
            if let wire::Event::Prices(prices) = &event {
                if let Some(market) = &prices.market {
                    if !market_details.contains_key(market) && market_details.len() == 2 {
                        return Err(Failure::Capacity);
                    }
                    market_details
                        .entry(market.clone())
                        .or_insert_with(|| [None, None])
                        [usize::from(prices.stream == metten_engine::sources::Stream::Trades)] =
                        prices.details.clone();
                } else {
                    details[prices.stream as usize] = prices.details.clone();
                }
            }
            let received = if let wire::Event::Prices(prices) = &event {
                Some((prices.stream, prices.market.clone()))
            } else {
                None
            };
            apply_source_event(state, number, event, &tokens, clock, lease, &mut consume)?;
            if let Some(progress) = &item.progress {
                progress.send_modify(|p| {
                    if let Some(stream) = acknowledged {
                        p.acknowledged[stream as usize] = true;
                    }
                    if let Some((stream, market)) = &received {
                        p.received(*stream, market.as_deref());
                    }
                });
            }
            Ok(())
        };
        let result = if expired {
            Err(Failure::Deadline)
        } else {
            match item.provider {
                Provider::Kalshi => {
                    self.kalshi
                        .decode(item.bytes.as_slice(), &self.tickers, &mut emit)
                }
                Provider::Coinbase => self.coinbase.decode(item.bytes.as_slice(), &mut emit),
                Provider::Kraken => self.kraken.decode(item.bytes.as_slice(), &mut emit),
            }
        };
        if self.blocked[metten_engine::sources::Stream::Book as usize] {
            self.kalshi.discard_book();
        }
        if let Err(error) = result {
            if let Some(stream) = wire_stream
                .filter(|_| !expired && matches!(error, Failure::Protocol | Failure::Capacity))
            {
                self.blocked[stream as usize] = true;
                let streams = if stream == metten_engine::sources::Stream::Metadata {
                    &MARKET_STREAMS[..]
                } else {
                    std::slice::from_ref(&stream)
                };
                revoke_gates(&self.gates, streams)?;
                consume(
                    self.state
                        .invalidate(
                            Scope::Stream(stream),
                            if error == Failure::Capacity {
                                Unavailable::Overflow
                            } else {
                                Unavailable::InvalidValue
                            },
                            at,
                            None,
                        )
                        .map_err(|_| Failure::Protocol)?,
                )?;
                if stream == metten_engine::sources::Stream::Book {
                    self.kalshi.discard_book();
                }
                if let Some(progress) = &item.progress {
                    progress.send_modify(|p| p.disable(stream));
                }
                if let (Some(sid), Some(outbound)) = (
                    self.kalshi.sid(stream),
                    &self.controls[item.provider.index()],
                ) {
                    self.command_id = self.command_id.checked_add(1).ok_or(Failure::Capacity)?;
                    outbound
                        .try_send(Control::KalshiUnsubscribe {
                            id: self.command_id,
                            subscriptions: vec![sid],
                        })
                        .map_err(|_| Failure::Capacity)?;
                }
                return Err(error);
            }
            item.fence.fail(error);
            revoke_gates(&self.gates, provider_streams(item.provider))?;
            let reason = match error {
                Failure::Capacity => Unavailable::Overflow,
                Failure::Deadline => Unavailable::OverAge,
                Failure::Transport => Unavailable::Gap,
                Failure::Denied => Unavailable::Access,
                Failure::Stopped => Unavailable::Stopped,
                _ => Unavailable::InvalidValue,
            };
            let effect = self
                .state
                .invalidate(
                    Scope::Provider(engine_provider(item.provider)),
                    reason,
                    at,
                    None,
                )
                .map_err(|_| Failure::Unavailable)?;
            consume(effect)?;
            return Err(error);
        }
        if !item.fence.current(item.generation) {
            consume(
                self.state
                    .invalidate(
                        Scope::Provider(engine_provider(item.provider)),
                        Unavailable::Disconnected,
                        at,
                        None,
                    )
                    .map_err(|_| Failure::Unavailable)?,
            )?;
            return Err(Failure::Stopped);
        }
        Ok(())
    }
}
pub struct SourceView {
    epoch_utc_ms: i64,
    blocked: [bool; 6],
    snapshot: metten_engine::sources::SourceSnapshot,
    details: [Option<wire::Details>; 6],
    market_details: BTreeMap<String, [Option<wire::Details>; 2]>,
    schedule: watch::Sender<Option<ScheduleLoss>>,
    worker_signal: Option<watch::Sender<bool>>,
    requirements: Vec<metten_engine::strategy::SourceRequirement>,
    fences: [Option<(Fence, u64)>; 3],
    epochs: [u64; 6],
    gates: [Arc<AtomicU64>; 6],
    _charge: tokio::sync::OwnedSemaphorePermit,
}
fn provider_streams(provider: Provider) -> &'static [metten_engine::sources::Stream] {
    use metten_engine::sources::Stream;
    match provider {
        Provider::Kalshi => &[Stream::Book, Stream::Trades, Stream::Brti, Stream::Metadata],
        Provider::Coinbase => &[Stream::Coinbase],
        Provider::Kraken => &[Stream::Kraken],
    }
}
fn revoke_gates(
    gates: &[Arc<AtomicU64>; 6],
    streams: &[metten_engine::sources::Stream],
) -> Result<(), Failure> {
    for stream in streams {
        gates[*stream as usize]
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |g| g.checked_add(1))
            .map_err(|_| Failure::Capacity)?;
    }
    Ok(())
}
impl SourceView {
    fn schedule_loss(&self, mut loss: ScheduleLoss) {
        self.schedule.send_modify(|pending| {
            if let Some(old) = *pending {
                if old.terminal {
                    loss.terminal = true;
                    loss.reason = old.reason;
                }
                if loss.epoch.is_none() {
                    loss.epoch = old.epoch;
                }
            }
            *pending = Some(loss);
        });
    }
    pub fn stream_current(&self, stream: metten_engine::sources::Stream) -> bool {
        if MARKET_STREAMS.contains(&stream) && self.schedule.borrow().is_some() {
            return false;
        }
        let i = stream as usize;
        let provider = match stream {
            metten_engine::sources::Stream::Coinbase => Provider::Coinbase,
            metten_engine::sources::Stream::Kraken => Provider::Kraken,
            _ => Provider::Kalshi,
        };
        let live = self.gates[i].load(Ordering::SeqCst);
        self.current(provider) && live != u64::MAX && live == self.epochs[i]
    }
    /// Recheck permissions at use, including a stop after this view was constructed.
    pub fn current(&self, provider: Provider) -> bool {
        if self.worker_signal.as_ref().is_some_and(|s| *s.borrow()) {
            return false;
        }
        self.fences[provider.index()]
            .as_ref()
            .is_none_or(|(f, g)| f.current(*g))
    }
    pub fn details(
        &self,
        stream: metten_engine::sources::Stream,
        market: Option<&str>,
    ) -> Result<Option<&wire::Details>, Failure> {
        if !self.stream_current(stream) {
            return Err(Failure::Stopped);
        }
        if matches!(
            stream,
            metten_engine::sources::Stream::Book | metten_engine::sources::Stream::Trades
        ) {
            let market = market.ok_or(Failure::InvalidRequest)?;
            Ok(self.market_details.get(market).and_then(|d| {
                d[usize::from(stream == metten_engine::sources::Stream::Trades)].as_ref()
            }))
        } else {
            if market.is_some() {
                return Err(Failure::InvalidRequest);
            }
            Ok(self.details[stream as usize].as_ref())
        }
    }
    pub fn snapshot(&self) -> Result<&metten_engine::sources::SourceSnapshot, Failure> {
        if self.schedule.borrow().is_some() {
            return Err(Failure::Stopped);
        }
        if self.worker_signal.as_ref().is_some_and(|s| *s.borrow()) {
            return Err(Failure::Stopped);
        }
        if self.gates.iter().enumerate().any(|(i, g)| {
            let live = g.load(Ordering::SeqCst);
            live == u64::MAX || live != self.epochs[i]
        }) {
            return Err(Failure::Stopped);
        }
        if self.fences.iter().flatten().any(|(f, g)| !f.current(*g)) {
            return Err(Failure::Stopped);
        }
        Ok(&self.snapshot)
    }
}
impl SourceWorker {
    fn observe_schedule_loss(&mut self, at: u64) -> Result<(), Failure> {
        let pending = { *self.schedule.borrow() };
        let Some(loss) = pending else { return Ok(()) };
        revoke_gates(&self.gates, &MARKET_STREAMS)?;
        self.kalshi.discard_book();
        self.state
            .invalidate(metten_engine::sources::Scope::Market, loss.reason, at, None)
            .map_err(|_| Failure::Protocol)?;
        if let Some(epoch) = loss.epoch {
            self.epoch_utc_ms = epoch;
        }
        if loss.terminal {
            for stream in MARKET_STREAMS {
                self.blocked[stream as usize] = true;
            }
            if let Some(progress) = &self.progress {
                progress.send_modify(|p| {
                    for stream in MARKET_STREAMS {
                        p.disable(stream);
                    }
                });
            }
        }
        self.schedule.send_if_modified(|pending| {
            if *pending == Some(loss) {
                *pending = None;
                true
            } else {
                false
            }
        });
        Ok(())
    }
    pub fn observe_stops(&mut self, at: u64) -> Result<(), Failure> {
        for provider in [Provider::Kalshi, Provider::Coinbase, Provider::Kraken] {
            if self.links[provider.index()]
                .as_ref()
                .is_some_and(|(f, g)| !f.current(*g))
            {
                let stream = provider_streams(provider)[0];
                self.connection(
                    provider,
                    self.state.token(stream),
                    metten_engine::sources::Connection::Disconnected,
                    at,
                )?;
            }
        }
        Ok(())
    }
}
pub(super) fn source_envelope(
    at: u64,
    event: metten_engine::sources::SourceEvent,
) -> metten_engine::sources::Envelope {
    metten_engine::sources::Envelope {
        clock: metten_engine::sources::Clock {
            at_ms: at,
            receipt_ms: at,
            received_through_ms: at,
            completed_frontier_ms: 0,
        },
        provider_identity: None,
        provider_time_utc: None,
        event,
    }
}
fn engine_provider(provider: Provider) -> metten_engine::sources::Provider {
    match provider {
        Provider::Kalshi => metten_engine::sources::Provider::Kalshi,
        Provider::Coinbase => metten_engine::sources::Provider::Coinbase,
        Provider::Kraken => metten_engine::sources::Provider::Kraken,
    }
}
fn apply_source_event(
    state: &mut metten_engine::sources::SourceState,
    number: &mut u64,
    event: wire::Event,
    tokens: &[metten_engine::sources::StreamToken; 6],
    clock: metten_engine::sources::Clock,
    lease: Option<u64>,
    consume: &mut impl FnMut(metten_engine::sources::SourceEffect) -> Result<(), Failure>,
) -> Result<(), Failure> {
    use metten_engine::{
        sources::*,
        strategy::{BindingScope, Source},
    };
    let at = clock.at_ms;
    match event {
        wire::Event::Heartbeat | wire::Event::ControlAcknowledged(_) => Ok(()),
        wire::Event::Absent(stream) => consume(
            state
                .invalidate(Scope::Stream(stream), Unavailable::Awaiting, at, None)
                .map_err(|_| Failure::Protocol)?,
        ),
        wire::Event::Gap(..) | wire::Event::ControlRejected(_) => Err(Failure::InvalidRequest),
        wire::Event::MetadataChanged => consume(
            state
                .invalidate(Scope::Market, Unavailable::MetadataChanged, at, None)
                .map_err(|_| Failure::Protocol)?,
        ),
        wire::Event::Acknowledged(stream, _) => {
            if tokens[stream as usize] != state.token(stream) {
                return Ok(());
            }
            consume(
                state
                    .apply(source_envelope(
                        at,
                        SourceEvent::Subscription {
                            stream,
                            token: tokens[stream as usize],
                            state: Subscription::Acknowledged,
                        },
                    ))
                    .map_err(|_| Failure::Protocol)?,
            )
        }
        wire::Event::Prices(prices) => {
            if tokens[prices.stream as usize] != state.token(prices.stream) {
                return Err(Failure::Stopped);
            }
            let view = state.snapshot(at).map_err(|_| Failure::Protocol)?;
            let source = match prices.stream {
                Stream::Book | Stream::Trades | Stream::Metadata => Source::Kalshi,
                Stream::Brti => Source::Brti,
                Stream::Coinbase => Source::Coinbase,
                Stream::Kraken => Source::Kraken,
            };
            let mut updates = Vec::new();
            for binding in view.bindings {
                if binding.requirement.source != source {
                    continue;
                }
                let (market, side) = match &binding.binding.scope {
                    BindingScope::Underlying => (None, None),
                    BindingScope::Market { market_id } => (Some(market_id.as_str()), None),
                    BindingScope::Contract {
                        market_id, side, ..
                    } => (Some(market_id.as_str()), Some(*side)),
                };
                if market != prices.market.as_deref() {
                    continue;
                }
                if let Some(reading) = prices
                    .readings
                    .iter()
                    .find(|r| r.field == binding.requirement.field && r.side == side)
                {
                    if let Some(value) = reading.value {
                        let value = if prices.initialize || !binding.initialized {
                            ValueUpdate::Initialize(value)
                        } else {
                            ValueUpdate::Observation {
                                number: number.checked_add(1).ok_or(Failure::Capacity)?,
                                value,
                            }
                        };
                        updates.push(BindingUpdate {
                            binding: binding.binding,
                            unit: binding.requirement.unit,
                            value: Some(value),
                            valid_until_ms: lease,
                        });
                    } else if binding.initialized {
                        // Expire this absent side without changing the live subscription identity.
                        updates.push(BindingUpdate {
                            binding: binding.binding,
                            unit: binding.requirement.unit,
                            value: None,
                            valid_until_ms: Some(0),
                        });
                    }
                }
            }
            if updates.is_empty() {
                return Ok(());
            }
            *number = number.checked_add(1).ok_or(Failure::Capacity)?;
            let mut envelope = source_envelope(
                at,
                SourceEvent::Update {
                    stream: prices.stream,
                    token: tokens[prices.stream as usize],
                    updates,
                    coverage: vec![],
                },
            );
            envelope.clock = clock;
            envelope.provider_identity = Some(prices.identity);
            envelope.provider_time_utc = prices.time;
            consume(state.apply(envelope).map_err(|_| Failure::Protocol)?)
        }
    }
}

/// Native-only handle. Step 5 may expose a finite projection, never this sender or fixtures.
pub(super) enum SourceCommand {
    Metadata {
        route: Route,
        bytes: memory::Bytes,
        epoch: i64,
        now: i64,
        token: metten_engine::sources::StreamToken,
        reply: tokio::sync::oneshot::Sender<Result<discovery::Parsed, Failure>>,
    },
    Markets {
        discovered: discovery::Discovered,
        token: metten_engine::sources::StreamToken,
        reply: tokio::sync::oneshot::Sender<Result<(), Failure>>,
    },
}
type Notice = (
    metten_engine::sources::StreamToken,
    metten_engine::sources::Connection,
    Option<Fence>,
);
type ControlSlot = Arc<std::sync::Mutex<Option<mpsc::Receiver<Control>>>>;
struct ControlOwner {
    slot: ControlSlot,
    receiver: Option<mpsc::Receiver<Control>>,
}
impl Drop for ControlOwner {
    fn drop(&mut self) {
        if let Ok(mut slot) = self.slot.lock() {
            *slot = self.receiver.take();
        }
    }
}
pub struct SourceHandle {
    pub(super) command: mpsc::Sender<SourceCommand>,
    epoch: Instant,
    memory: memory::Memory,
    pub(super) schedule: watch::Sender<Option<ScheduleLoss>>,
    controls: [ControlSlot; 3],
    outbound: [mpsc::Sender<Control>; 3],
    notices: [watch::Sender<Option<Notice>>; 3],
    pub intake: mpsc::Sender<Intake>,
    pub latest: watch::Receiver<Option<Arc<SourceView>>>,
    _base_charge: Arc<tokio::sync::OwnedSemaphorePermit>,
}
impl Coordinator {
    pub fn start_sources(&mut self, mut source: SourceWorker) -> Result<SourceHandle, Failure> {
        if self.stopping || self.worker.is_some() {
            return Err(Failure::Stopped);
        }
        source.worker_signal = Some(self.stop.clone());
        let base_charge = source._base_charge.clone();
        let memory = source.memory.clone();
        let epoch = source.epoch;
        let schedule = source.schedule.clone();
        let channels: [_; 3] = std::array::from_fn(|_| mpsc::channel(7));
        let mut receivers: [ControlSlot; 3] =
            std::array::from_fn(|_| Arc::new(std::sync::Mutex::new(None)));
        let mut senders = Vec::new();
        for (i, (tx, rx)) in channels.into_iter().enumerate() {
            source.controls[i] = Some(tx.clone());
            senders.push(tx);
            receivers[i] = Arc::new(std::sync::Mutex::new(Some(rx)));
        }
        let outbound: [mpsc::Sender<Control>; 3] =
            senders.try_into().map_err(|_| Failure::Unavailable)?;
        let notices: [watch::Sender<Option<Notice>>; 3] =
            std::array::from_fn(|_| watch::channel(None).0);
        let notice_readers = notices.clone();
        let (command, mut command_rx) = mpsc::channel(1);
        let (tx, mut rx) = intake();
        let (publish, latest) = watch::channel(None);
        let stop = self.stop.subscribe();
        let lifetime = WorkerLifetime(self.stop.clone());
        self.worker = Some(std::thread::spawn(move || {
            let _lifetime = lifetime;
            while !*stop.borrow() {
                let mut at = match u64::try_from(epoch.elapsed().as_millis()) {
                    Ok(at) => at,
                    Err(_) => return,
                };
                if source.observe_schedule_loss(at).is_err() {
                    return;
                }
                for provider in [Provider::Kalshi, Provider::Coinbase, Provider::Kraken] {
                    if let Some((token, state, session)) =
                        notice_readers[provider.index()].send_replace(None)
                        && source.connection(provider, token, state, at).is_ok()
                        && let Some(session) = session
                    {
                        source.links[provider.index()] = Some((session.clone(), session.token()));
                    }
                }
                if source.observe_stops(at).is_err() || source.advance(at, |_| Ok(())).is_err() {
                    return;
                }
                if let Ok(command) = command_rx.try_recv() {
                    match command {
                        SourceCommand::Markets {
                            discovered,
                            token,
                            reply,
                        } => {
                            let result = if reply.is_closed() {
                                Err(Failure::Stopped)
                            } else {
                                source.install_markets(&discovered.markets, token, at, |_| Ok(()))
                            };
                            let _ = reply.send(result);
                        }
                        SourceCommand::Metadata {
                            route,
                            bytes,
                            epoch,
                            now,
                            token,
                            reply,
                        } => {
                            if !reply.is_closed() {
                                let result = if token
                                    != source.state.token(metten_engine::sources::Stream::Metadata)
                                {
                                    Err(Failure::Stopped)
                                } else {
                                    discovery::parse(
                                        &route,
                                        bytes.as_slice(),
                                        epoch,
                                        now,
                                        &source.memory,
                                    )
                                };
                                let _ = reply.send(result);
                            }
                        }
                    }
                }
                match rx.try_recv() {
                    Ok(item) => {
                        at = at.max(u64::try_from(epoch.elapsed().as_millis()).unwrap_or(u64::MAX));
                        if source.observe_schedule_loss(at).is_err()
                            || source.observe_stops(at).is_err()
                            || source.advance(at, |_| Ok(())).is_err()
                        {
                            return;
                        }
                        let _ = source.process(item, at, 0, |_| Ok(()));
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => return,
                    Err(mpsc::error::TryRecvError::Empty) => (),
                }
                if source.observe_schedule_loss(at).is_err() {
                    return;
                }
                match source.snapshot(at) {
                    Ok(view) => {
                        publish.send_replace(Some(Arc::new(view)));
                    }
                    Err(Failure::Stopped) if source.schedule.borrow().is_some() => (),
                    Err(_) => return,
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            let _ = source
                .state
                .stop(u64::try_from(epoch.elapsed().as_millis()).unwrap_or(u64::MAX));
        }));
        Ok(SourceHandle {
            intake: tx,
            latest,
            memory,
            epoch,
            schedule,
            command,
            controls: receivers,
            outbound,
            notices,
            _base_charge: base_charge,
        })
    }
}

impl SourceWorker {
    /// A discovery completion retains the metadata token captured before its request.
    pub fn install_markets(
        &mut self,
        markets: &[metten_engine::strategy::MarketDescriptor],
        token: metten_engine::sources::StreamToken,
        at: u64,
        mut consume: impl FnMut(metten_engine::sources::SourceEffect) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        use metten_engine::{sources::*, strategy::BindingScope};
        if markets.len() > 2
            || token != self.state.token(Stream::Metadata)
            || markets
                .iter()
                .any(|m| i128::from(m.close_ms) <= i128::from(at))
        {
            return Err(Failure::Stopped);
        }
        for descriptor in markets {
            let before = self.state.snapshot(at).map_err(|_| Failure::Protocol)?;
            if let Some(old) = before
                .markets
                .iter()
                .find(|m| m.descriptor.market_id == descriptor.market_id)
            {
                if old.descriptor != *descriptor {
                    self.kalshi.discard_book();
                    revoke_gates(
                        &self.gates,
                        &[Stream::Book, Stream::Trades, Stream::Metadata],
                    )?;
                    let effect = self
                        .state
                        .apply(source_envelope(
                            at,
                            SourceEvent::ReplaceMarket {
                                token,
                                expected: old.descriptor.clone(),
                                replacement: descriptor.clone(),
                            },
                        ))
                        .map_err(|_| Failure::Protocol)?;
                    consume(effect)?;
                    // Replacement deliberately needs a new request, not a re-labelled old response.
                    return Err(Failure::Stopped);
                }
            } else {
                let mut next = before
                    .bindings
                    .iter()
                    .map(|b| b.binding.binding_id)
                    .max()
                    .unwrap_or(0);
                let mut bindings = Vec::new();
                for requirement in self
                    .requirements
                    .iter()
                    .filter(|r| r.instrument != "btc_usd")
                {
                    use metten_engine::strategy::{BindingDescriptor, Side, SourceSide};
                    let scopes = if requirement.instrument == "current_market" {
                        vec![BindingScope::Market {
                            market_id: descriptor.market_id.clone(),
                        }]
                    } else {
                        let sides = match requirement.side {
                            Some(SourceSide::Selected) => vec![Side::Yes, Side::No],
                            Some(SourceSide::Fixed(s)) => vec![s],
                            None => return Err(Failure::Protocol),
                        };
                        sides
                            .into_iter()
                            .map(|side| BindingScope::Contract {
                                market_id: descriptor.market_id.clone(),
                                contract_id: descriptor.contract_id.clone(),
                                side,
                            })
                            .collect()
                    };
                    for scope in scopes {
                        next = next.checked_add(1).ok_or(Failure::Capacity)?;
                        bindings.push(BindingDescriptor {
                            binding_id: next,
                            input_id: requirement.input_id.clone(),
                            scope,
                        });
                    }
                }
                #[cfg(test)]
                let capabilities = if self.lease.is_some() {
                    bindings
                        .iter()
                        .map(|b| {
                            (
                                b.binding_id,
                                Capabilities {
                                    current: None,
                                    observations: None,
                                    history: None,
                                },
                            )
                        })
                        .collect()
                } else {
                    vec![]
                };
                #[cfg(not(test))]
                let capabilities = vec![];
                consume(
                    self.state
                        .apply(source_envelope(
                            at,
                            SourceEvent::RegisterMarket {
                                token,
                                descriptor: descriptor.clone(),
                                bindings,
                                capabilities,
                            },
                        ))
                        .map_err(|_| Failure::Protocol)?,
                )?;
            }
            consume(
                self.state
                    .apply(source_envelope(
                        at,
                        SourceEvent::Metadata {
                            token,
                            market_id: descriptor.market_id.clone(),
                            verified: true,
                        },
                    ))
                    .map_err(|_| Failure::Protocol)?,
            )?;
        }
        self.refresh_tickers(at)?;
        if let Some(progress) = &self.progress {
            progress.send_modify(|p| {
                p.tokens[Stream::Metadata as usize] = Some(self.state.token(Stream::Metadata))
            });
            let trade_token = self.state.token(Stream::Trades);
            let sent_trade_token = { progress.borrow().tokens[Stream::Trades as usize] };
            if sent_trade_token != Some(trade_token)
                && !self.tickers.is_empty()
                && !self.blocked[Stream::Trades as usize]
                && let Some(sid) = self.trade_sid
            {
                self.command_id = self.command_id.checked_add(1).ok_or(Failure::Capacity)?;
                progress.send_modify(|p| {
                    p.recover(Control::KalshiTradeUpdate {
                        id: self.command_id,
                        sid,
                        action: BookAction::Add,
                        markets: self.tickers.clone(),
                        token: trade_token,
                    })
                });
            }
            let token = self.state.token(Stream::Book);
            let sent_book_token = { progress.borrow().tokens[Stream::Book as usize] };
            if sent_book_token != Some(token)
                && !self.tickers.is_empty()
                && !self.blocked[Stream::Book as usize]
                && let Some(sid) = self.book_sid
            {
                self.command_id = self.command_id.checked_add(1).ok_or(Failure::Capacity)?;
                progress.send_modify(|p| p.desired_book = Some(token));
                progress.send_modify(|p| {
                    p.recover(Control::KalshiBookUpdate {
                        id: self.command_id,
                        sid,
                        action: BookAction::Snapshot,
                        markets: self.tickers.clone(),
                        token: Some(token),
                    })
                });
            }
        }
        Ok(())
    }
    fn refresh_tickers(&mut self, at: u64) -> Result<(), Failure> {
        use metten_engine::{sources::Stream, strategy::Source};
        let tickers: Vec<_> = self
            .state
            .snapshot(at)
            .map_err(|_| Failure::Protocol)?
            .markets
            .into_iter()
            .map(|m| m.descriptor.market_id)
            .collect();
        let added: Vec<_> = tickers
            .iter()
            .filter(|t| !self.tickers.contains(t))
            .cloned()
            .collect();
        let removed: Vec<_> = self
            .tickers
            .iter()
            .filter(|t| !tickers.contains(t))
            .cloned()
            .collect();
        self.kalshi.retain(&tickers);
        self.market_details.retain(|id, _| tickers.contains(id));
        if let Some(outbound) = &self.controls[0] {
            for (stream, sid, channel) in [
                (Stream::Book, self.book_sid, Channel::Book),
                (Stream::Trades, self.trade_sid, Channel::Trade),
            ] {
                let needed = self.requirements.iter().any(|r| {
                    r.source == Source::Kalshi
                        && r.instrument != "current_market"
                        && (r.field == "last_trade") == (stream == Stream::Trades)
                });
                if !needed || self.blocked[stream as usize] {
                    continue;
                }
                for (markets, action) in [(&removed, BookAction::Delete), (&added, BookAction::Add)]
                {
                    if markets.is_empty() {
                        continue;
                    }
                    self.command_id = self.command_id.checked_add(1).ok_or(Failure::Capacity)?;
                    let token = self.state.token(stream);
                    let control = match sid {
                        Some(sid) if stream == Stream::Book => Control::KalshiBookUpdate {
                            id: self.command_id,
                            sid,
                            action,
                            markets: markets.clone(),
                            token: Some(token),
                        },
                        Some(sid) => Control::KalshiTradeUpdate {
                            id: self.command_id,
                            sid,
                            action,
                            markets: markets.clone(),
                            token,
                        },
                        None if matches!(action, BookAction::Add) => Control::KalshiSubscribe {
                            id: self.command_id,
                            channel,
                            markets: tickers.clone(),
                        },
                        None => continue,
                    };
                    outbound.try_send(control).map_err(|_| Failure::Capacity)?;
                }
            }
        }
        self.tickers = tickers;
        Ok(())
    }
    pub fn advance(
        &mut self,
        at: u64,
        mut consume: impl FnMut(metten_engine::sources::SourceEffect) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        if at < self.at {
            return Err(Failure::InvalidRequest);
        }
        let mut view = self.state.snapshot(at).map_err(|_| Failure::Protocol)?;
        if view
            .markets
            .iter()
            .any(|m| i128::from(m.descriptor.close_ms) <= i128::from(at))
        {
            revoke_gates(&self.gates, &MARKET_STREAMS)?;
        }
        view.markets.sort_by_key(|m| m.descriptor.open_ms);
        for market in &view.markets {
            let Ok(opening) = u64::try_from(market.descriptor.open_ms) else {
                continue;
            };
            if self.at <= opening
                && opening <= at
                && i128::from(at) < i128::from(market.descriptor.close_ms)
            {
                // Never give post-opening readiness or invalidity an earlier source clock.
                match self.state.snapshot(opening) {
                    Ok(_) => (),
                    Err(error) if error.code == "invalid_request" => continue,
                    Err(_) => return Err(Failure::Protocol),
                }
                consume(
                    self.state
                        .apply(source_envelope(
                            opening,
                            metten_engine::sources::SourceEvent::Boundary {
                                market_id: market.descriptor.market_id.clone(),
                            },
                        ))
                        .map_err(|_| Failure::Protocol)?,
                )?;
            }
        }
        if let Some(market) = view.markets.iter().find(|m| {
            m.verified
                && i128::from(m.descriptor.open_ms) < i128::from(at)
                && i128::from(at) < i128::from(m.descriptor.close_ms)
                && view.current_market.as_deref() != Some(m.descriptor.market_id.as_str())
        }) {
            consume(
                self.state
                    .apply(source_envelope(
                        at,
                        metten_engine::sources::SourceEvent::Metadata {
                            token: self.state.token(metten_engine::sources::Stream::Metadata),
                            market_id: market.descriptor.market_id.clone(),
                            verified: true,
                        },
                    ))
                    .map_err(|_| Failure::Protocol)?,
            )?;
        }
        consume(
            self.state
                .apply(source_envelope(
                    at,
                    metten_engine::sources::SourceEvent::Advance,
                ))
                .map_err(|_| Failure::Protocol)?,
        )?;
        self.at = at;
        self.refresh_tickers(at)
    }
}

impl Coordinator {
    /// Explicit native setup only; startup and screen refresh never call this.
    pub fn start_provider(
        &mut self,
        provider: Provider,
        transport: Arc<transport::Transport>,
        handle: &mut SourceHandle,
        credential: Option<(String, signing::Key)>,
    ) -> Result<(), Failure> {
        if !handle.memory.same_owner(transport.memory()) {
            return Err(Failure::InvalidRequest);
        }
        let slot = handle.controls[provider.index()].clone();
        let receiver = slot
            .lock()
            .map_err(|_| Failure::Unavailable)?
            .take()
            .ok_or(Failure::InvalidRequest)?;
        let mut controls = ControlOwner {
            slot,
            receiver: Some(receiver),
        };
        let outbound = handle.outbound[provider.index()].clone();
        let intake = handle.intake.clone();
        let mut latest = handle.latest.clone();
        let metadata_command = handle.command.clone();
        let epoch = handle.epoch;
        let notice = handle.notices[provider.index()].clone();
        let parent = self.fence(provider);
        let mut stop = self.cancellation(provider);
        let base_charge = handle._base_charge.clone();
        self.spawn(provider, async move {
            let _base_charge = base_charge;
            let mut obsolete = None;
            let mut metadata_validated = false;
            let context_for = |view: &Arc<SourceView>| {
                let v = &view.snapshot;
                let tokens = SOURCE_STREAMS.map(|s| {
                    v.streams
                        .iter()
                        .find(|(stream, _, _)| *stream == s)
                        .unwrap()
                        .2
                });
                let tickers = v
                    .markets
                    .iter()
                    .map(|m| m.descriptor.market_id.clone())
                    .collect::<Vec<_>>();
                let mut streams = Vec::new();
                for requirement in &view.requirements {
                    let stream = match requirement.source {
                        metten_engine::strategy::Source::Kalshi
                            if requirement.instrument == "current_market" =>
                        {
                            metten_engine::sources::Stream::Metadata
                        }
                        metten_engine::strategy::Source::Kalshi
                            if requirement.field == "last_trade" =>
                        {
                            metten_engine::sources::Stream::Trades
                        }
                        metten_engine::strategy::Source::Kalshi => {
                            metten_engine::sources::Stream::Book
                        }
                        metten_engine::strategy::Source::Brti => {
                            metten_engine::sources::Stream::Brti
                        }
                        metten_engine::strategy::Source::Coinbase => {
                            metten_engine::sources::Stream::Coinbase
                        }
                        metten_engine::strategy::Source::Kraken => {
                            metten_engine::sources::Stream::Kraken
                        }
                    };
                    if !view.blocked[stream as usize] && !streams.contains(&stream) {
                        streams.push(stream)
                    }
                }
                (tokens, tickers, streams, view.epoch_utc_ms)
            };
            loop {
                if !parent.current(parent.token()) || *stop.borrow() {
                    return;
                }
                let context = loop {
                    let context = latest
                        .borrow()
                        .as_ref()
                        .filter(|v| v.snapshot().is_ok())
                        .map(&context_for);
                    if let Some(context) = context {
                        let stream = match provider {
                            Provider::Kalshi => metten_engine::sources::Stream::Book,
                            Provider::Coinbase => metten_engine::sources::Stream::Coinbase,
                            Provider::Kraken => metten_engine::sources::Stream::Kraken,
                        };
                        if obsolete != Some(context.0[stream as usize]) {
                            break context;
                        }
                    }
                    if bounded(&mut stop, Instant::now() + SHUTDOWN, async {
                        latest.changed().await.map_err(|_| Failure::Stopped)
                    })
                    .await
                    .is_err()
                    {
                        return;
                    }
                };
                let (tokens, _, streams, _) = context;
                let connection_stream = match provider {
                    Provider::Kalshi => metten_engine::sources::Stream::Book,
                    Provider::Coinbase => metten_engine::sources::Stream::Coinbase,
                    Provider::Kraken => metten_engine::sources::Stream::Kraken,
                };
                if !provider_streams(provider)
                    .iter()
                    .any(|s| streams.contains(s))
                {
                    return;
                }
                let session = match parent.session() {
                    Ok(session) => session,
                    Err(_) => return,
                };
                while controls.receiver.as_mut().unwrap().try_recv().is_ok() {}
                let material = credential.as_ref().map(|(id, key)| (id.as_str(), key));
                let deadline = Instant::now() + Duration::from_secs(10);
                let connected = transport
                    .connect(provider, material, &mut stop, deadline, &session)
                    .await;
                let connection = match connected {
                    Ok(connection) => connection,
                    Err(error) => {
                        use metten_engine::sources::Connection;
                        obsolete = Some(tokens[connection_stream as usize]);
                        notice.send_replace(Some((
                            tokens[connection_stream as usize],
                            match error {
                                Failure::Denied => Connection::AccessRefused,
                                Failure::Missing => Connection::MissingConfiguration,
                                _ => Connection::Disconnected,
                            },
                            None,
                        )));
                        if matches!(error, Failure::Transport | Failure::Deadline)
                            && parent.current(parent.token())
                        {
                            continue;
                        } else {
                            return;
                        }
                    }
                };
                let fresh = latest
                    .borrow()
                    .as_ref()
                    .filter(|v| v.snapshot().is_ok())
                    .map(&context_for);
                let Some((fresh_tokens, tickers, streams, epoch_utc_ms)) = fresh else {
                    return;
                };
                if provider_streams(provider)
                    .iter()
                    .any(|s| fresh_tokens[*s as usize] != tokens[*s as usize])
                {
                    drop(connection);
                    obsolete = None;
                    continue;
                }
                let source_clock = (epoch, epoch_utc_ms);
                let mut commands = Vec::new();
                match provider {
                    Provider::Kalshi => {
                        for (stream, channel) in [
                            (metten_engine::sources::Stream::Book, Channel::Book),
                            (metten_engine::sources::Stream::Trades, Channel::Trade),
                            (metten_engine::sources::Stream::Brti, Channel::Brti),
                        ] {
                            if streams.contains(&stream)
                                && (!matches!(channel, Channel::Book | Channel::Trade)
                                    || !tickers.is_empty())
                            {
                                commands.push(Control::KalshiSubscribe {
                                    id: commands.len() as u32 + 1,
                                    channel,
                                    markets: if matches!(channel, Channel::Book | Channel::Trade) {
                                        tickers.clone()
                                    } else {
                                        vec![]
                                    },
                                })
                            }
                        }
                        if streams.iter().any(|s| {
                            matches!(
                                s,
                                metten_engine::sources::Stream::Book
                                    | metten_engine::sources::Stream::Trades
                                    | metten_engine::sources::Stream::Metadata
                            )
                        }) {
                            commands.push(Control::KalshiSubscribe {
                                id: commands.len() as u32 + 1,
                                channel: Channel::Lifecycle,
                                markets: vec![],
                            })
                        }
                    }
                    Provider::Coinbase if streams.contains(&connection_stream) => {
                        commands.push(Control::Coinbase { subscribe: true })
                    }
                    Provider::Kraken if streams.contains(&connection_stream) => {
                        commands.push(Control::Kraken { subscribe: true })
                    }
                    _ => return,
                }
                if commands.is_empty() {
                    return;
                }
                notice.send_replace(Some((
                    tokens[connection_stream as usize],
                    metten_engine::sources::Connection::Open,
                    Some(session.clone()),
                )));
                for command in commands {
                    if outbound.try_send(command).is_err() {
                        session.fail(Failure::Capacity);
                        return;
                    }
                }
                let service = transport.serve(
                    connection,
                    controls.receiver.as_mut().unwrap(),
                    intake.clone(),
                    tokens,
                    stop.clone(),
                    Some(watch::channel(transport::Progress::new()).0),
                );
                let metadata = async {
                    if provider == Provider::Kalshi
                        && streams.iter().any(|s| {
                            matches!(
                                s,
                                metten_engine::sources::Stream::Book
                                    | metten_engine::sources::Stream::Trades
                                    | metten_engine::sources::Stream::Metadata
                            )
                        })
                    {
                        run_metadata(
                            &transport,
                            latest.clone(),
                            metadata_command.clone(),
                            session.clone(),
                            stop.clone(),
                            source_clock,
                            &mut metadata_validated,
                        )
                        .await
                    } else {
                        std::future::pending::<Result<(), Failure>>().await
                    }
                };
                let result = tokio::try_join!(service, metadata).map(|_| ());
                let reason = session
                    .cause
                    .lock()
                    .ok()
                    .and_then(|cause| *cause)
                    .or(result.err());
                if !matches!(reason, Some(Failure::Transport | Failure::Deadline))
                    || !parent.current(parent.token())
                {
                    return;
                }
                obsolete = Some(tokens[connection_stream as usize]);
                notice.send_replace(Some((
                    tokens[connection_stream as usize],
                    metten_engine::sources::Connection::Disconnected,
                    None,
                )));
            }
        })
    }
}

impl SourceWorker {
    pub fn prepare(
        strategy: &metten_engine::strategy::ValidatedStrategy,
        markets: &[metten_engine::strategy::MarketDescriptor],
        memory: memory::Memory,
    ) -> Result<Self, Failure> {
        use metten_engine::strategy::{BindingDescriptor, BindingScope, Side, SourceSide};
        if markets.len() > 2 {
            return Err(Failure::Capacity);
        }
        for market in markets {
            if [&market.series_id, &market.market_id, &market.contract_id]
                .iter()
                .any(|s| s.len() > 128 || s.capacity() > 128)
            {
                return Err(Failure::Capacity);
            }
        }
        let _construction = memory.reserve(64 * 1024)?;
        let requirements = strategy.source_requirements();
        let count: usize = requirements
            .iter()
            .map(|r| {
                if r.instrument == "btc_usd" {
                    1
                } else {
                    markets.len()
                        * if r.side == Some(SourceSide::Selected) {
                            2
                        } else {
                            1
                        }
                }
            })
            .sum();
        if count > 128 {
            return Err(Failure::Capacity);
        }
        let mut bindings = Vec::with_capacity(count);
        for requirement in requirements {
            if requirement.instrument == "btc_usd" {
                bindings.push(BindingDescriptor {
                    binding_id: bindings.len() as u32 + 1,
                    input_id: requirement.input_id,
                    scope: BindingScope::Underlying,
                });
                continue;
            }
            for market in markets {
                if requirement.instrument == "current_market" {
                    bindings.push(BindingDescriptor {
                        binding_id: bindings.len() as u32 + 1,
                        input_id: requirement.input_id.clone(),
                        scope: BindingScope::Market {
                            market_id: market.market_id.clone(),
                        },
                    });
                    continue;
                }
                let sides = match requirement.side {
                    Some(SourceSide::Selected) => vec![Side::Yes, Side::No],
                    Some(SourceSide::Fixed(side)) => vec![side],
                    None => return Err(Failure::InvalidRequest),
                };
                for side in sides {
                    bindings.push(BindingDescriptor {
                        binding_id: bindings.len() as u32 + 1,
                        input_id: requirement.input_id.clone(),
                        scope: BindingScope::Contract {
                            market_id: market.market_id.clone(),
                            contract_id: market.contract_id.clone(),
                            side,
                        },
                    })
                }
            }
        }
        let source = metten_engine::sources::SourceState::new(strategy, markets, &bindings, &[])
            .map_err(|_| Failure::InvalidRequest)?;
        let mut worker = Self::new(source, memory)?;
        worker.requirements = strategy.source_requirements();
        Ok(worker)
    }
}

async fn run_metadata(
    transport: &transport::Transport,
    latest: watch::Receiver<Option<Arc<SourceView>>>,
    command: mpsc::Sender<SourceCommand>,
    fence: Fence,
    mut stop: watch::Receiver<bool>,
    source_clock: (Instant, i64),
    metadata_validated: &mut bool,
) -> Result<(), Failure> {
    use metten_engine::sources::Stream;
    let mut epoch_utc = source_clock.1;
    let mut startup = !*metadata_validated;
    let mut previous_count = 0;
    let mut checks: Option<(i64, discovery::PreparationChecks)> = None;
    let mut attempted_metadata = None;
    let mut previous = (Instant::now(), utc_now_ms()?);
    let mut missing_after_open = None;
    loop {
        if *stop.borrow() || !fence.current(fence.token()) {
            return Err(Failure::Stopped);
        }
        let now = utc_now_ms()?;
        let instant = Instant::now();
        let elapsed = i64::try_from(instant.saturating_duration_since(previous.0).as_millis())
            .map_err(|_| Failure::Unavailable)?;
        let wall = now.checked_sub(previous.1).ok_or(Failure::Unavailable)?;
        let discontinuity = clock_discontinuity(elapsed, wall, true);
        previous = (instant, now);
        let view = latest.borrow().clone();
        if let Some(view) = view {
            if discontinuity {
                let at = i64::try_from(source_clock.0.elapsed().as_millis())
                    .map_err(|_| Failure::Unavailable)?;
                epoch_utc = now.checked_sub(at).ok_or(Failure::Unavailable)?;
                view.schedule_loss(ScheduleLoss {
                    epoch: Some(epoch_utc),
                    reason: metten_engine::sources::Unavailable::MetadataChanged,
                    terminal: false,
                });
                if let Some((_, checks)) = &mut checks {
                    checks.skip_through(now - epoch_utc);
                }
                attempted_metadata = None;
            } else if !view.blocked[Stream::Metadata as usize]
                && let Ok(snapshot) = view.snapshot()
            {
                let token = snapshot
                    .streams
                    .iter()
                    .find(|(s, _, _)| *s == Stream::Metadata)
                    .unwrap()
                    .2;
                let relative = now.checked_sub(epoch_utc).ok_or(Failure::Unavailable)?;
                let open = snapshot
                    .markets
                    .iter()
                    .map(|m| m.descriptor.close_ms)
                    .min()
                    .unwrap_or(0);
                if checks.as_ref().is_none_or(|(o, _)| *o != open) {
                    checks = Some((open, discovery::PreparationChecks::new(open, relative)));
                }
                let missing = snapshot.markets.len() < 2;
                let unverified: Vec<_> = snapshot
                    .markets
                    .iter()
                    .filter(|m| !m.verified)
                    .map(|m| m.descriptor.clone())
                    .collect();
                let needs_work = missing || !unverified.is_empty();
                let retired = previous_count == 2 && snapshot.markets.len() < 2;
                previous_count = snapshot.markets.len();
                let due = checks
                    .as_mut()
                    .is_some_and(|(_, c)| c.due(relative, !needs_work, false, false));
                let post_open = relative >= open;
                let permitted = !post_open || missing_after_open.is_none_or(|t| instant >= t);
                let request = startup
                    || (needs_work && (due || retired) && permitted)
                    || (!unverified.is_empty() && attempted_metadata != Some(token) && permitted);
                if request {
                    attempted_metadata = Some(token);
                    if post_open {
                        missing_after_open = Some(instant + Duration::from_secs(60));
                    }
                    let result = if !startup && !unverified.is_empty() {
                        discovery::revalidate(
                            transport,
                            &mut stop,
                            &fence,
                            &unverified,
                            epoch_utc,
                            now,
                            &command,
                            token,
                        )
                        .await
                    } else {
                        discovery::pair(
                            transport, &mut stop, &fence, now, epoch_utc, startup, &command, token,
                        )
                        .await
                    };
                    match result {
                        Ok(discovered) => {
                            startup = false;
                            *metadata_validated = true;
                            let (reply, receive) = tokio::sync::oneshot::channel();
                            let deadline = Instant::now() + SHUTDOWN;
                            let installed = bounded(&mut stop, deadline, async {
                                command
                                    .send(SourceCommand::Markets {
                                        discovered,
                                        token,
                                        reply,
                                    })
                                    .await
                                    .map_err(|_| Failure::Stopped)?;
                                receive.await.map_err(|_| Failure::Stopped)
                            })
                            .await?;
                            match installed {
                                Ok(()) => {
                                    startup = false;
                                    *metadata_validated = true;
                                }
                                Err(Failure::Stopped | Failure::Protocol) => (),
                                Err(error) => return Err(error),
                            }
                        }
                        Err(Failure::Budget | Failure::Unavailable | Failure::Deadline) => {
                            startup = false;
                        }
                        Err(Failure::Stopped) => (),
                        Err(error) => {
                            use metten_engine::sources::Unavailable;
                            view.schedule_loss(ScheduleLoss {
                                epoch: None,
                                reason: match error {
                                    Failure::Denied => Unavailable::Access,
                                    Failure::Capacity => Unavailable::Overflow,
                                    _ => Unavailable::InvalidValue,
                                },
                                terminal: true,
                            });
                            return std::future::pending::<Result<(), Failure>>().await;
                        }
                    }
                }
            }
        }
        let end = (Instant::now(), utc_now_ms()?);
        let elapsed = i64::try_from(end.0.saturating_duration_since(previous.0).as_millis())
            .map_err(|_| Failure::Unavailable)?;
        let wall = end.1.checked_sub(previous.1).ok_or(Failure::Unavailable)?;
        if clock_discontinuity(elapsed, wall, false)
            && let Some(view) = latest.borrow().as_ref()
        {
            epoch_utc = end
                .1
                .checked_sub(
                    i64::try_from(source_clock.0.elapsed().as_millis())
                        .map_err(|_| Failure::Unavailable)?,
                )
                .ok_or(Failure::Unavailable)?;
            view.schedule_loss(ScheduleLoss {
                epoch: Some(epoch_utc),
                reason: metten_engine::sources::Unavailable::MetadataChanged,
                terminal: false,
            });
            attempted_metadata = None;
        }
        previous = end;
        wait(
            &mut stop,
            Instant::now() + Duration::from_secs(2),
            Duration::from_secs(1),
        )
        .await?;
    }
}

fn clock_discontinuity(monotonic_ms: i64, utc_delta_ms: i64, idle_wait: bool) -> bool {
    let limit = MAX_INTAKE_AGE.as_millis() as u64;
    utc_delta_ms < 0
        || utc_delta_ms.abs_diff(monotonic_ms) > limit
        || (idle_wait && monotonic_ms as u64 > limit)
}

fn utc_now_ms() -> Result<i64, Failure> {
    i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| Failure::Unavailable)?
            .as_millis(),
    )
    .map_err(|_| Failure::Unavailable)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exhausted_native_epoch_permanently_revokes_views_without_wrapping() {
        let strategy = metten_engine::strategy::ValidatedStrategy::parse(include_str!(
            "../../../examples/learning.json"
        ))
        .unwrap();
        let worker = SourceWorker::prepare(&strategy, &[], memory::Memory::default()).unwrap();
        worker.gates[0].store(u64::MAX - 1, Ordering::SeqCst);
        let old = worker.snapshot(0).unwrap();
        revoke_gates(&worker.gates, &SOURCE_STREAMS[..1]).unwrap();
        assert!(!old.stream_current(metten_engine::sources::Stream::Book));
        assert!(old.snapshot().is_err());
        assert!(matches!(worker.snapshot(0), Err(Failure::Capacity)));
        assert_eq!(
            revoke_gates(&worker.gates, &SOURCE_STREAMS[..1]),
            Err(Failure::Capacity)
        );
    }
    #[test]
    fn pending_schedule_loss_cannot_be_relabelled_as_a_current_snapshot() {
        use metten_engine::sources::{Stream, Unavailable};
        let strategy = metten_engine::strategy::ValidatedStrategy::parse(include_str!(
            "../../../examples/learning.json"
        ))
        .unwrap();
        let mut worker = SourceWorker::prepare(
            &strategy,
            &super::super::tests::source_markets(),
            memory::Memory::default(),
        )
        .unwrap();
        worker
            .install_markets(
                &super::super::tests::source_markets(),
                worker.tokens()[Stream::Metadata as usize],
                0,
                |_| Ok(()),
            )
            .unwrap();
        let before = worker.snapshot(0).unwrap();
        assert!(
            before
                .snapshot()
                .unwrap()
                .markets
                .iter()
                .all(|m| m.verified)
        );
        before.schedule_loss(ScheduleLoss {
            epoch: Some(0),
            reason: Unavailable::MetadataChanged,
            terminal: false,
        });
        assert!(!before.stream_current(Stream::Book));
        assert!(before.stream_current(Stream::Brti));
        assert!(before.snapshot().is_err());
        assert!(
            matches!(worker.snapshot(0), Err(Failure::Stopped)),
            "new publication cannot capture the unapplied permission generation"
        );
        worker.observe_schedule_loss(0).unwrap();
        assert!(!before.stream_current(Stream::Metadata));
        let after = worker.snapshot(0).unwrap();
        assert!(
            after
                .snapshot()
                .unwrap()
                .markets
                .iter()
                .all(|m| !m.verified)
        );
        assert_eq!(after.epoch_utc_ms, 0);
    }
    #[test]
    fn ordinary_metadata_work_is_not_a_sleep_event_and_terminal_facts_are_not_lost() {
        use metten_engine::sources::Unavailable;
        assert!(!clock_discontinuity(6000, 6000, false));
        assert!(clock_discontinuity(6000, 6000, true));
        assert!(clock_discontinuity(100, 10000, false));
        assert!(clock_discontinuity(100, -1, false));
        let strategy = metten_engine::strategy::ValidatedStrategy::parse(include_str!(
            "../../../examples/learning.json"
        ))
        .unwrap();
        let worker = SourceWorker::prepare(&strategy, &[], memory::Memory::default()).unwrap();
        let view = worker.snapshot(0).unwrap();
        view.schedule_loss(ScheduleLoss {
            epoch: None,
            reason: Unavailable::Access,
            terminal: true,
        });
        view.schedule_loss(ScheduleLoss {
            epoch: Some(123),
            reason: Unavailable::MetadataChanged,
            terminal: false,
        });
        let loss = worker.schedule.borrow().unwrap();
        assert!(loss.terminal);
        assert_eq!(loss.reason, Unavailable::Access);
        assert_eq!(loss.epoch, Some(123));
    }
}
