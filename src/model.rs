use serde::Deserialize;

/// K线数据 (不可变结构体)
#[derive(Debug, Clone, Deserialize)]
pub struct KLine {
    pub time: i64,     // 时间戳
    pub open: f64,     // 开盘价
    pub high: f64,     // 最高价
    pub low: f64,      // 最低价
    pub close: f64,    // 收盘价
    pub volume: i32,   // 成交量
}

/// Tick 数据结构
#[derive(Debug, Clone, Deserialize)]
pub struct TickData {
    pub time: i64,         // 时间戳
    pub last_price: f64,   // 最新成交价
    pub volume: i32,       // 成交量
    
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
}


/// 持仓信息
#[derive(Debug, Default)]
pub struct Position {
    pub code: String,
    pub name: String,
    /// 持仓数量
    pub volume: i32,
    /// 可用数量
    pub available_vol: i32,
    /// 当前价格
    pub current_price: f64,
    /// 成本价
    pub cost_price: f64,
    /// 盈亏
    pub profit: f64,
}


/// 交割单
#[derive(Debug)]
pub struct Transaction {
    /// 成交时间
    pub time: i64,
    /// 成交价格
    pub price: f64,
    /// 成交数量
    pub volume: i32,
    /// 交易类型（买入/卖出） B S
    pub order_type: char,
    /// 成交后数量
    pub remain_vol: i32,
    /// 成交后成本价
    pub remain_cost: f64,
}

/// 委托
#[derive(Debug, Clone)]
pub struct Order {
    /// 市场
    pub market_type: char,
    /// 股票代码
    pub code: String,
    /// 委托时间
    pub time: i64,
    /// 委托价格
    pub price: f64,
    /// 委托数量
    pub volume: i32,
    /// 委托类型 B S
    pub order_type: char,  
}
