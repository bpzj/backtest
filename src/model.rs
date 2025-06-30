use serde::Deserialize;

/// K线数据 (不可变结构体)
#[derive(Debug, Clone, Deserialize)]
pub struct KLine {
    pub time: i64,     // 时间戳
    pub open: f64,     // 开盘价
    pub high: f64,     // 最高价
    pub low: f64,      // 最低价
    pub close: f64,    // 收盘价
    /// 成交量, i32 会溢出
    pub volume: i64,
}

/// Tick 数据结构
#[derive(Debug, Clone, Deserialize)]
pub struct TickData {
    pub time: i64,         // 时间戳
    pub last_price: f64,   // 最新成交价
    pub volume: i32,       // 成交量

    // 卖盘
    pub ask1_price: f64,
    pub ask1_volume: i32,
    pub ask2_price: f64,
    pub ask2_volume: i32,
    pub ask3_price: f64,
    pub ask3_volume: i32,
    pub ask4_price: f64,
    pub ask4_volume: i32,
    pub ask5_price: f64,
    pub ask5_volume: i32,
    
    // 买盘
    pub bid1_price: f64,
    pub bid1_volume: i32,
    pub bid2_price: f64,
    pub bid2_volume: i32,
    pub bid3_price: f64,
    pub bid3_volume: i32,
    pub bid4_price: f64,
    pub bid4_volume: i32,
    pub bid5_price: f64,
    pub bid5_volume: i32,
}

