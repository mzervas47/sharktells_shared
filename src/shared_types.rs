use serde::{Serialize, Deserialize};
use crate::SettlementCurrency;
use crate::QuoteCurrency;


//pub quote: String ??
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq, bincode::Decode, bincode::Encode)]
pub struct SymbolPair {
    pub canonical: String,
    pub raw: String,    
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub enum ContractType {
    Linear,   // quote margined (e.g. USDT)
    Inverse,  // base margined (e.g. BTC)
    Spot,     // no leverage
}

pub type ExchangeKey = (Exchange, SymbolPair, ContractType, SettlementCurrency);

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct PriceUpdate {
    pub exchange: Exchange,           // The originating venue (Binance, Bybit, Kraken, etc.) — identifies source of truth for this tick.
    pub symbol: SymbolPair,           // The trading pair identifier normalized to each exchange’s convention (e.g., BTCUSDT, PI_XBTUSD, ETH-PERPETUAL).
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency,
    pub bid: f64,                     // The highest price currently available for buyers (best bid) — top-of-book buy price.
    pub ask: f64,                     // The lowest price currently available for sellers (best ask) — top-of-book sell price.
    pub exchange_timestamp: u64,      // Canonical UTC-aligned timestamp (ms) of the update — either exchange engine time or normalized system time.
    pub engine_timestamp: u64,
    pub data_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct DepthSnapshot {
    pub exchange: Exchange,
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency, 
    pub bids: Vec<[f64; 2]>,          // [price, quantity]
    pub asks: Vec<[f64; 2]>,          // [price, quantity]
    pub exchange_timestamp: u64,
    pub engine_timestamp: u64,
    pub data_source: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct LiquidationEvent {
    pub exchange: Exchange,   // Originating venue — identifies which exchange triggered the liquidation (e.g., Binance, Bybit, Kraken, OKX, Deribit).
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency,
    pub side: Side,           // Direction of the forced liquidation: Side::Buy → short position liquidated (shorts being forced to buy back), Side::Sell → long position liquidated (longs being forced to sell)
    pub price: f64,           // Actual execution price of the liquidation fill (midpoint or mark where the forced order executed).
    pub quantity: f64,        // Total notional or contract size liquidated (base units, not quote value); normalized to f64 for uniform precision.
    pub raw_quantity: f64,
    pub exchange_timestamp: u64,       // Canonical event timestamp in epoch milliseconds, exchange-sourced if available, otherwise system-aligned.
    pub engine_timestamp: u64,
    pub data_source: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct FundingRateUpdate {
    pub exchange: Exchange,
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency,
    pub rate: f64, // Funding rate (e.g., 0.0001 = 0.01%)
    pub interval_ms: u64,  // Funding interval duration in milliseconds
    pub next_funding_time: u64,// Epoch ms of the next scheduled funding
    pub exchange_timestamp: u64,        // Epoch ms when this update was emitted
    pub engine_timestamp: u64,
    pub data_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct MarkPriceUpdate {
    pub exchange: Exchange,
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency,
    pub mark_price: f64,  // Current mark price used for funding calculation
    pub exchange_timestamp: u64,
    pub engine_timestamp: u64,
    pub data_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct OpenInterestUpdate {
    pub exchange: Exchange,
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency,
    pub open_interest_native: f64, // Total number of outstanding contracts
    pub open_interest_usd: f64,
    pub change: Option<f64>,   // Delta since last update (positive or negative)
    pub exchange_timestamp: u64, // Epoch ms
    pub engine_timestamp: u64,
    pub data_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct TradeUpdate {
    pub exchange: Exchange,
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency,
    pub price: f64,
    pub quantity: f64,
    pub side: Side,           // unified buy/sell direction
    pub is_buyer_maker: bool, // raw flag from exchange
    pub exchange_timestamp: u64,      // epoch ms
    pub engine_timestamp: u64,
    pub data_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct KlineUpdate {
    pub exchange: Exchange,
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency, 
    pub interval: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub start_time: u64,
    pub close_time: u64,
    pub is_closed: bool,
    pub engine_timestamp: u64,
    pub data_source: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, bincode::Decode, bincode::Encode)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, bincode::Decode, bincode::Encode)]
pub enum Exchange {
    Binance,
    Bybit,
    Kucoin,
    Kraken,
    Deribit,
    Okx,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub enum EngineIngressMessage {
    Price(PriceUpdate),
    Depth(DepthSnapshot),
    Funding(FundingRateUpdate),
    MarkPrice(MarkPriceUpdate),
    OpenInterest(OpenInterestUpdate),
    Trade(TradeUpdate),
    Kline(KlineUpdate),
    Liquidation(LiquidationSnapshot),
}

impl EngineIngressMessage {
    pub fn topic(&self) -> SocketTopic {
        match self {
            EngineIngressMessage::Price(_) => SocketTopic::Price,
            EngineIngressMessage::Depth(_) => SocketTopic::Depth,
            EngineIngressMessage::Funding(_) => SocketTopic::Funding,
            EngineIngressMessage::MarkPrice(_) => SocketTopic::MarkPrice,
            EngineIngressMessage::OpenInterest(_) => SocketTopic::OpenInterest,
            EngineIngressMessage::Trade(_) => SocketTopic::Trades,
            EngineIngressMessage::Kline(_) => SocketTopic::Klines,
            EngineIngressMessage::Liquidation(_) => SocketTopic::Liquidations,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)]
pub struct IngressEnvelope {
    pub version: u16,
    pub message: EngineIngressMessage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SocketTopic {
    Price,
    Depth,
    Funding,
    MarkPrice,
    OpenInterest,
    Trades,
    Klines,
    Liquidations,
    Debug,
}

impl SocketTopic {
    pub fn name(&self) -> &'static str {
        match self {
            SocketTopic::Price => "price",
            SocketTopic::Depth => "depth",
            SocketTopic::Liquidations => "liquidations",
            SocketTopic::OpenInterest => "open_interest",
            SocketTopic::Funding => "funding",
            SocketTopic::MarkPrice => "mark_price",
            SocketTopic::Trades => "trades",
            SocketTopic::Klines => "klines",
            SocketTopic::Debug => "debug",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Decode, bincode::Encode)] 
pub struct LiquidationSnapshot {
    pub exchange: Exchange,
    pub symbol: SymbolPair,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency, 
    pub window_start: u64, // start of current aggregation window (epoch ms)
    pub window_len: u64, // fixed window length in ms
    pub buy_count: u64,
    pub sell_count: u64,
    pub total_buy_qty: f64,
    pub total_sell_qty: f64,
    pub raw_total_buy_qty: f64,
    pub raw_total_sell_qty: f64,
    pub exchange_timestamp: u64,  // last contributing event exchange time
    pub engine_timestamp: u64,    // last contributing event engine time
}

impl LiquidationSnapshot {
    pub fn new(event: &LiquidationEvent) -> Self {
        Self {
            exchange: event.exchange,
            symbol: event.symbol.clone(),
            contract_type: event.contract_type,
            settlement_currency: event.settlement_currency,
            window_start: event.exchange_timestamp,
            window_len: 300_000,
            buy_count: 0,
            sell_count: 0,
            total_buy_qty: 0.0,
            total_sell_qty: 0.0,
            raw_total_buy_qty: 0.0,
            raw_total_sell_qty: 0.0,
            exchange_timestamp: event.exchange_timestamp,
            engine_timestamp: event.engine_timestamp,
        }
    }

    pub fn update(&mut self, event: &LiquidationEvent) {
        if event.exchange_timestamp.saturating_sub(self.window_start) > self.window_len {
            self.window_start = event.exchange_timestamp;
            self.buy_count = 0;
            self.sell_count = 0;
            self.total_buy_qty = 0.0;
            self.total_sell_qty = 0.0;
            self.raw_total_buy_qty = 0.0;
            self.raw_total_sell_qty = 0.0;
        }

        self.exchange_timestamp = event.exchange_timestamp;
        self.engine_timestamp = event.engine_timestamp;

        match event.side {
            Side::Buy => {
                self.buy_count += 1;
                self.total_buy_qty += event.quantity;
                self.raw_total_buy_qty += event.raw_quantity;
            }
            Side::Sell => {
                self.sell_count += 1;
                self.total_sell_qty += event.quantity;
                self.raw_total_sell_qty += event.raw_quantity;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentSnapshot {
    pub exchange: Exchange,
    pub raw_symbol: String,
    pub canonical_symbol: String,
    pub contract_type: ContractType,
    pub settlement_currency: SettlementCurrency,
    pub quote_currency: QuoteCurrency,
    pub contract_size: f64,
    pub tick_size: f64,
    pub lot_size: f64,
    pub socket: SocketKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SocketKind {
    BinancePublic { instance: u32 }, 
    BybitPublic  { instance: u32 },
    KucoinPublic { instance: u32 },
    KrakenPublic { instance: u32 },
    DeribitPublic { instance: u32 },
    OkxPublic { instance: u32 },    
    OkxBusiness { instance: u32 }, 
    BinancePublicSpot { instance: u32 }, 
    BybitPublicSpot { instance: u32 }, 
    OkxPublicSpot { instance: u32 }, 
    DeribitPublicSpot { instance: u32 }, 
    KrakenPublicSpot { instance: u32 }, 
    DeribitPublicInverse { instance: u32 }, 
    BinancePublicInverse {instance: u32 }, 
    BybitPublicInverse {instance: u32 }, 
}

