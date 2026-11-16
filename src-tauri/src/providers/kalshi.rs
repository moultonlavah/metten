//! Fixed Kalshi channels; books and provider identities never enter strategy code.
use super::{Failure, memory::Memory, wire::*};
use metten_engine::{sources::Stream, strategy::Side};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use tokio::sync::OwnedSemaphorePermit;
const BOOK_BYTES: usize = 2 * 8192 * 128 + 65536;
#[derive(Deserialize)]
struct Message<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
    sid: Option<u64>,
    id: Option<u32>,
    sending_ts_ms: Option<u64>,
    seq: Option<u64>,
    #[serde(borrow)]
    msg: Option<&'a RawValue>,
}
#[derive(Deserialize)]
struct Payload<'a> {
    market_ticker: Option<&'a str>,
    clock: Option<u64>,
    received_at: Option<u64>,
    #[serde(borrow)]
    yes_dollars_fp: Option<&'a RawValue>,
    #[serde(borrow)]
    no_dollars_fp: Option<&'a RawValue>,
    #[serde(borrow)]
    price_dollars: Option<&'a RawValue>,
    #[serde(borrow)]
    delta_fp: Option<&'a RawValue>,
    side: Option<&'a str>,
    trade_id: Option<&'a str>,
    #[serde(borrow)]
    yes_price_dollars: Option<&'a RawValue>,
    #[serde(borrow)]
    no_price_dollars: Option<&'a RawValue>,
    ts_ms: Option<u64>,
    #[serde(borrow)]
    count_fp: Option<&'a RawValue>,
    taker_side: Option<&'a str>,
    is_block_trade: Option<bool>,
    #[serde(borrow)]
    avg_60s_data: Option<&'a RawValue>,
    #[serde(borrow)]
    last_60s_windowed_average_15min: Option<&'a RawValue>,
    index_id: Option<&'a str>,
    #[serde(borrow)]
    data: Option<&'a RawValue>,
}
#[derive(Deserialize)]
struct Index<'a> {
    id: &'a str,
    time: u64,
    #[serde(borrow)]
    value: &'a RawValue,
}
struct Book {
    yes: BTreeMap<i128, i128>,
    no: BTreeMap<i128, i128>,
    _charge: OwnedSemaphorePermit,
}
pub struct Kalshi {
    memory: Memory,
    books: BTreeMap<String, Book>,
    sequences: BTreeMap<u64, u64>,
    sids: BTreeMap<u64, Stream>,
    trades: BTreeMap<String, BTreeMap<String, (i128, i128, OwnedSemaphorePermit)>>,
    index: Trades,
    index_available: Option<bool>,
}
impl Kalshi {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            books: BTreeMap::new(),
            sequences: BTreeMap::new(),
            sids: BTreeMap::new(),
            trades: BTreeMap::new(),
            index: Trades::default(),
            index_available: None,
        }
    }
    pub fn retain(&mut self, tickers: &[String]) {
        self.books.retain(|k, _| tickers.contains(k));
        self.trades.retain(|k, _| tickers.contains(k));
    }
    pub fn reconnect(&mut self) {
        self.books.clear();
        self.sequences.clear();
        self.sids.clear();
        self.index_available = None;
        self.index.reconnect()
    }
    fn sequence(&mut self, sid: u64, seq: u64) -> Result<bool, Failure> {
        if sid == 0 {
            return Err(Failure::Protocol);
        }
        if let Some(last) = self.sequences.get(&sid) {
            if seq <= *last {
                return Ok(false);
            }
            if last.checked_add(1) != Some(seq) {
                if self
                    .sids
                    .get(&sid)
                    .is_none_or(|s| matches!(s, Stream::Book | Stream::Metadata))
                {
                    self.books.clear();
                }
                if self.sids.get(&sid) == Some(&Stream::Brti) {
                    self.index.reconnect();
                    self.index_available = None;
                }
                self.sequences.insert(sid, seq);
                return Err(Failure::Transport);
            }
        }
        if !self.sequences.contains_key(&sid) && self.sequences.len() == 4 {
            return Err(Failure::Capacity);
        }
        self.sequences.insert(sid, seq);
        Ok(true)
    }
    fn stream_kind(kind: &str) -> Option<Stream> {
        match kind {
            "orderbook_snapshot" | "orderbook_delta" => Some(Stream::Book),
            "trade" => Some(Stream::Trades),
            "cfbenchmarks_value" | "cfbenchmarks_value_indexlist" => Some(Stream::Brti),
            "market_lifecycle_v2" => Some(Stream::Metadata),
            _ => None,
        }
    }
    pub fn skip_obsolete(&mut self, bytes: &[u8]) -> Result<(), Failure> {
        let m: Message<'_> = decode(bytes)?;
        if let (Some(sid), Some(seq)) = (m.sid, m.seq)
            && let Some(last) = self.sequences.get_mut(&sid)
        {
            *last = (*last).max(seq)
        }
        Ok(())
    }
    pub fn stream(&self, bytes: &[u8]) -> Result<Option<Stream>, Failure> {
        let message: Message<'_> = decode(bytes)?;
        let stream = Self::stream_kind(message.kind);
        if let Some(stream) = stream
            && self.sids.get(&message.sid.ok_or(Failure::Protocol)?) != Some(&stream)
        {
            return Err(Failure::Protocol);
        }
        Ok(stream)
    }
    pub fn sid(&self, stream: Stream) -> Option<u32> {
        self.sids
            .iter()
            .find(|(_, s)| **s == stream)
            .and_then(|(sid, _)| u32::try_from(*sid).ok())
    }
    pub fn discard_book(&mut self) {
        self.books.clear();
    }

    pub fn decode(
        &mut self,
        bytes: &[u8],
        tickers: &[String],
        mut emit: impl FnMut(Event) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let m: Message<'_> = decode(bytes)?;
        if matches!(m.kind, "ok" | "unsubscribed") {
            if let Some(sid) = m.sid {
                let stream = *self.sids.get(&sid).ok_or(Failure::Protocol)?;
                match self.sequence(sid, m.seq.ok_or(Failure::Protocol)?) {
                    Ok(false) => return Ok(()),
                    Ok(true) => (),
                    Err(Failure::Transport) => {
                        return emit(Event::Gap(
                            stream,
                            u32::try_from(sid).map_err(|_| Failure::Capacity)?,
                        ));
                    }
                    Err(error) => return Err(error),
                }
            } else if m.kind == "unsubscribed" {
                return Err(Failure::Protocol);
            }
            return emit(Event::ControlAcknowledged(m.id.ok_or(Failure::Protocol)?));
        }
        if m.kind == "error" {
            #[derive(Deserialize)]
            struct Rejected {
                code: u32,
            }
            let rejected: Rejected = decode(m.msg.ok_or(Failure::Protocol)?.get().as_bytes())?;
            let _ = rejected.code; // Unknown provider codes are not inferred to mean entitlement denial.
            return emit(Event::ControlRejected(m.id.ok_or(Failure::Protocol)?));
        }
        if m.kind == "subscribed" {
            #[derive(Deserialize)]
            struct Ack<'a> {
                channel: &'a str,
                sid: u64,
            }
            let ack: Ack<'_> = decode(m.msg.ok_or(Failure::Protocol)?.get().as_bytes())?;
            let stream = match ack.channel {
                "orderbook_delta" => Stream::Book,
                "trade" => Stream::Trades,
                "cfbenchmarks_value" => Stream::Brti,
                "market_lifecycle_v2" => Stream::Metadata,
                _ => return Err(Failure::Protocol),
            };
            if ack.sid == 0 || (!self.sids.contains_key(&ack.sid) && self.sids.len() == 4) {
                return Err(Failure::Capacity);
            }
            if let Some((sid, _)) = self.sids.iter().find(|(_, s)| **s == stream) {
                if *sid != ack.sid {
                    return Err(Failure::Protocol);
                }
                return emit(Event::Acknowledged(
                    stream,
                    u32::try_from(ack.sid).map_err(|_| Failure::Capacity)?,
                ));
            }
            self.sequences
                .retain(|sid, _| self.sids.get(sid) != Some(&stream));
            self.sids.retain(|_, s| *s != stream);
            self.sids.insert(ack.sid, stream);
            return emit(Event::Acknowledged(
                stream,
                u32::try_from(ack.sid).map_err(|_| Failure::Capacity)?,
            ));
        }
        let stream = Self::stream_kind(m.kind).ok_or(Failure::Protocol)?;
        if self.sids.get(&m.sid.ok_or(Failure::Protocol)?) != Some(&stream) {
            return Err(Failure::Protocol);
        }
        match self.sequence(
            m.sid.ok_or(Failure::Protocol)?,
            m.seq.ok_or(Failure::Protocol)?,
        ) {
            Ok(false) => return Ok(()),
            Ok(true) => (),
            Err(Failure::Transport) => {
                return emit(Event::Gap(
                    stream,
                    u32::try_from(m.sid.unwrap()).map_err(|_| Failure::Capacity)?,
                ));
            }
            Err(error) => return Err(error),
        }
        if m.kind == "cfbenchmarks_value_indexlist" {
            #[derive(Deserialize)]
            struct Available<'a> {
                #[serde(borrow)]
                index_ids: List<&'a str>,
            }
            let available: Available<'_> =
                decode(m.msg.ok_or(Failure::Protocol)?.get().as_bytes())?;
            for id in &available.index_ids.0 {
                identity(id)?;
            }
            let found = available.index_ids.0.contains(&"BRTI");
            self.index_available = Some(found);
            return if found {
                emit(Event::Heartbeat)
            } else {
                emit(Event::Absent(Stream::Brti))
            };
        }
        let p: Payload<'_> = decode(m.msg.ok_or(Failure::Protocol)?.get().as_bytes())?;
        match m.kind {
            "orderbook_snapshot" | "orderbook_delta" => {
                let ticker = p.market_ticker.ok_or(Failure::Protocol)?;
                identity(ticker)?;
                if !tickers.iter().any(|t| t == ticker) {
                    return Ok(());
                }
                if m.kind == "orderbook_snapshot" {
                    if !self.books.contains_key(ticker) && self.books.len() == 2 {
                        return Err(Failure::Capacity);
                    }
                    let charge = self.memory.reserve(BOOK_BYTES)?;
                    let yes = levels(p.yes_dollars_fp.ok_or(Failure::Protocol)?)?;
                    let no = levels(p.no_dollars_fp.ok_or(Failure::Protocol)?)?;
                    self.books.insert(
                        ticker.into(),
                        Book {
                            yes,
                            no,
                            _charge: charge,
                        },
                    );
                } else {
                    let Some(book) = self.books.get_mut(ticker) else {
                        return Ok(());
                    };
                    let levels = match p.side {
                        Some("yes") => &mut book.yes,
                        Some("no") => &mut book.no,
                        _ => return Err(Failure::Protocol),
                    };
                    let price = cents(p.price_dollars.ok_or(Failure::Protocol)?)?.millionths();
                    let delta = number(p.delta_fp.ok_or(Failure::Protocol)?)?.millionths();
                    let quantity = levels
                        .get(&price)
                        .copied()
                        .unwrap_or(0)
                        .checked_add(delta)
                        .ok_or(Failure::Protocol)?;
                    scaled(quantity)?;
                    if quantity < 0 {
                        return Err(Failure::Protocol);
                    }
                    if quantity == 0 {
                        levels.remove(&price);
                    } else {
                        if !levels.contains_key(&price) && levels.len() == 8192 {
                            return Err(Failure::Capacity);
                        }
                        levels.insert(price, quantity);
                    }
                }
                let book = self.books.get(ticker).unwrap();
                let yes = book.yes.last_key_value().map(|(p, _)| *p);
                let no = book.no.last_key_value().map(|(p, _)| *p);
                let values = [
                    ("bid", Side::Yes, yes),
                    ("bid", Side::No, no),
                    ("buyable_ask", Side::Yes, no.map(|n| 100_000_000 - n)),
                    ("buyable_ask", Side::No, yes.map(|n| 100_000_000 - n)),
                ];
                let mut readings = Vec::with_capacity(4);
                for (field, side, value) in values {
                    readings.push(Reading {
                        field,
                        side: Some(side),
                        value: value.map(scaled).transpose()?,
                    });
                }
                emit(Event::Prices(Prices {
                    stream: Stream::Book,
                    market: Some(ticker.into()),
                    readings,
                    identity: format!("{}:{}", m.sid.unwrap(), m.seq.unwrap()),
                    time: None,
                    initialize: m.kind == "orderbook_snapshot",
                    details: Some(Details::Book {
                        clock: p.clock,
                        sid: m.sid.unwrap(),
                        sequence: m.seq.unwrap(),
                        quantities: [
                            book.yes.last_key_value().map(|(_, q)| *q),
                            book.no.last_key_value().map(|(_, q)| *q),
                            book.no.last_key_value().map(|(_, q)| *q),
                            book.yes.last_key_value().map(|(_, q)| *q),
                        ]
                        .map(|q| q.map(scaled).transpose())
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()?
                        .try_into()
                        .map_err(|_| Failure::Protocol)?,
                    }),
                }))
            }
            "trade" => {
                let ticker = p.market_ticker.ok_or(Failure::Protocol)?;
                if !tickers.iter().any(|t| t == ticker) {
                    return Ok(());
                }
                if p.taker_side.is_some_and(|s| !matches!(s, "yes" | "no")) {
                    return Err(Failure::Protocol);
                }
                if let Some(q) = p.count_fp {
                    quantity_text(q)?;
                }
                let id = p.trade_id.ok_or(Failure::Protocol)?;
                identity(id)?;
                let yes = cents(p.yes_price_dollars.ok_or(Failure::Protocol)?)?;
                let no = cents(p.no_price_dollars.ok_or(Failure::Protocol)?)?;
                let ledger = self.trades.entry(ticker.into()).or_default();
                if let Some((y, n, _)) = ledger.get(id) {
                    return if *y == yes.millionths() && *n == no.millionths() {
                        Ok(())
                    } else {
                        Err(Failure::Protocol)
                    };
                }
                let charge = self.memory.reserve(id.len() + 2048)?;
                let initial = ledger.is_empty();
                ledger.insert(id.into(), (yes.millionths(), no.millionths(), charge));
                emit(Event::Prices(Prices {
                    stream: Stream::Trades,
                    market: Some(ticker.into()),
                    readings: vec![
                        Reading {
                            field: "last_trade",
                            side: Some(Side::Yes),
                            value: Some(yes),
                        },
                        Reading {
                            field: "last_trade",
                            side: Some(Side::No),
                            value: Some(no),
                        },
                    ],
                    identity: id.into(),
                    time: p.ts_ms.map(|t| t.to_string()),
                    initialize: initial,
                    details: Some(Details::Trade {
                        quantity: p.count_fp.map(quantity_text).transpose()?,
                        direction: p.taker_side.map(str::to_owned),
                        role: "taker",
                        sequence: m.seq,
                        block: p.is_block_trade,
                    }),
                }))
            }
            "cfbenchmarks_value" => {
                if p.index_id != Some("BRTI") {
                    return Err(Failure::Protocol);
                }
                let raw = p.data.ok_or(Failure::Protocol)?;
                let text: String =
                    serde_json::from_str(raw.get()).map_err(|_| Failure::Protocol)?;
                let tick: Index<'_> = decode(text.as_bytes())?;
                if tick.id != "BRTI" {
                    return Err(Failure::Protocol);
                }
                let value = usd(tick.value)?;
                if self.index_available != Some(true) {
                    return emit(Event::Heartbeat);
                }
                if let Some(initialize) = self.index.accept(tick.time, value, false)? {
                    emit(Event::Prices(Prices {
                        stream: Stream::Brti,
                        market: None,
                        readings: vec![Reading {
                            field: "index_value",
                            side: None,
                            value: Some(value),
                        }],
                        identity: tick.time.to_string(),
                        time: Some(tick.time.to_string()),
                        initialize,
                        details: Some(Details::Brti {
                            received_at: p.received_at,
                            sending_ts_ms: m.sending_ts_ms,
                            trailing: average(p.avg_60s_data)?,
                            final_minute: average(p.last_60s_windowed_average_15min)?,
                        }),
                    }))?
                }
                Ok(())
            }
            "market_lifecycle_v2" => {
                if p.market_ticker
                    .is_some_and(|ticker| tickers.iter().any(|t| t == ticker))
                {
                    self.books.clear();
                    emit(Event::MetadataChanged)
                } else {
                    emit(Event::Heartbeat)
                }
            }
            _ => Err(Failure::Protocol),
        }
    }
}
fn levels(raw: &RawValue) -> Result<BTreeMap<i128, i128>, Failure> {
    let rows: List<(&RawValue, &RawValue)> = decode(raw.get().as_bytes())?;
    let mut map = BTreeMap::new();
    for (p, q) in rows.0 {
        let price = cents(p)?.millionths();
        let quantity = number(q)?.millionths();
        if quantity <= 0 || map.insert(price, quantity).is_some() {
            return Err(Failure::Protocol);
        }
    }
    Ok(map)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn books_are_replaced_and_shared_gap_discards_both() {
        let memory = Memory::default();
        let mut p = Kalshi::new(memory.clone());
        let tickers = vec!["KXBTC15M-M".into(), "KXBTC15M-N".into()];
        let mut asks = vec![];
        p.decode(
            br#"{"type":"subscribed","msg":{"sid":1,"channel":"orderbook_delta"}}"#,
            &tickers,
            |_| Ok(()),
        )
        .unwrap();
        for (seq, ticker) in [(1, "KXBTC15M-M"), (2, "KXBTC15M-N")] {
            let bytes = format!(
                r#"{{"type":"orderbook_snapshot","sid":1,"seq":{seq},"msg":{{"market_ticker":"{ticker}","yes_dollars_fp":[["0.36","2"]],"no_dollars_fp":[["0.64","3"]]}}}}"#
            );
            p.decode(bytes.as_bytes(), &tickers, |e| {
                if let Event::Prices(p) = e {
                    asks.push(p.readings[2].value.unwrap().millionths())
                }
                Ok(())
            })
            .unwrap();
        }
        assert_eq!(asks, vec![36_000_000; 2]);
        let mut gap = false;
        p.decode(br#"{"type":"orderbook_delta","sid":1,"seq":4,"msg":{"market_ticker":"KXBTC15M-M","side":"yes","price_dollars":"0.36","delta_fp":"1"}}"#,&tickers,|event|{gap=matches!(event,Event::Gap(Stream::Book,1));Ok(())}).unwrap();
        assert!(gap);
        assert!(p.books.is_empty());
        assert_eq!(memory.used(), 0);
    }
    #[test]
    fn exact_book_capacity_replacement_overlap_and_refusal_preserve_reservations() {
        let memory = Memory::default();
        let mut p = Kalshi::new(memory.clone());
        let tickers = vec!["KXBTC15M-M".into()];
        p.decode(
            br#"{"type":"subscribed","msg":{"sid":1,"channel":"orderbook_delta"}}"#,
            &tickers,
            |_| Ok(()),
        )
        .unwrap();
        let rows: Vec<_> = (0..8192).map(|i| format!(r#"["0.{i:06}","1"]"#)).collect();
        let input = format!(
            r#"{{"type":"orderbook_snapshot","sid":1,"seq":1,"msg":{{"market_ticker":"KXBTC15M-M","yes_dollars_fp":[{}],"no_dollars_fp":[]}}}}"#,
            rows.join(",")
        );
        p.decode(input.as_bytes(), &tickers, |_| Ok(())).unwrap();
        assert_eq!(p.books["KXBTC15M-M"].yes.len(), 8192);
        assert_eq!(memory.used(), BOOK_BYTES);
        let held = memory
            .reserve(32 * super::super::MAX_BYTES - 2 * BOOK_BYTES + 1)
            .unwrap();
        let replacement = input.replace("\"seq\":1", "\"seq\":2");
        assert_eq!(
            p.decode(replacement.as_bytes(), &tickers, |_| Ok(())),
            Err(Failure::Capacity)
        );
        assert_eq!(p.books["KXBTC15M-M"].yes.len(), 8192);
        drop(held);
        p.reconnect();
        assert_eq!(memory.used(), 0);
        let too_many = format!("[{},[\"0.9\",\"1\"]]", rows.join(","));
        assert!(matches!(
            levels(decode::<&RawValue>(too_many.as_bytes()).unwrap()),
            Err(Failure::Capacity)
        ));
        const { assert!(3 * BOOK_BYTES < 8 * super::super::MAX_BYTES) };
    }
    #[test]
    fn brti_absence_is_not_denial_and_raw_tick_stays_separate_from_averages() {
        let mut p = Kalshi::new(Memory::default());
        p.decode(
            br#"{"type":"subscribed","msg":{"sid":3,"channel":"cfbenchmarks_value"}}"#,
            &[],
            |_| Ok(()),
        )
        .unwrap();
        let mut absent = false;
        p.decode(
            br#"{"type":"cfbenchmarks_value_indexlist","sid":3,"seq":1,"msg":{"index_ids":[]}}"#,
            &[],
            |e| {
                absent = matches!(e, Event::Absent(Stream::Brti));
                Ok(())
            },
        )
        .unwrap();
        assert!(absent);
        p.decode(br#"{"type":"cfbenchmarks_value_indexlist","sid":3,"seq":2,"msg":{"index_ids":["BRTI"]}}"#,&[],|_|Ok(())).unwrap();
        let mut raw = None;
        p.decode(br#"{"type":"cfbenchmarks_value","sid":3,"seq":3,"msg":{"index_id":"BRTI","data":"{\"id\":\"BRTI\",\"time\":1,\"value\":\"68000.12\"}","avg_60s_data":{"value":"69000.12345678","window_size":14}}}"#,&[],|e|{if let Event::Prices(p)=e {raw=p.readings[0].value;assert!(matches!(p.details,Some(Details::Brti {trailing:Some(Average{count:14,..}),..})));}Ok(())}).unwrap();
        assert_eq!(raw.unwrap().millionths(), 68000120000);
    }
}
