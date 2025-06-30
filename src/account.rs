use std::collections::HashMap;
use std::collections::hash_map::Entry;



// 账户主体
#[derive(Debug,Default)]
pub struct Account {
    pub name: String,
    // 记录资金、盈亏
    // pub assets: Assets,
    /// 总资产 = 总市值 + 可用金额 + 冻结金额
    pub balance: f64,
    /// 冻结金额 （未成交的委托占用）
    pub freeze_balance: f64,
    /// 可用金额
    pub available_balance: f64,

    /// 总市值
    pub portfolio_value: f64,
    /// 总盈亏
    pub profit: f64,
    /// 持仓
    pub hold: HashMap<StockCode, Position>,

    /// 交割单
    pub transactions: Vec<Transaction>,
    /// 订单记录
    pub orders: Vec<Order>,
    
    // 佣金费率 
    // commission_ratio:f64,
    // 税率
    // tax_ratio:f64,
}


impl Account {
    /// 买入操作
    pub fn buy(&mut self, order: &Order) -> bool {
        let turnover = order.price * order.volume as f64;
        // 资金检查
        if self.available_balance < turnover {
            return false;
        }
        // 更新资产
        self.available_balance = self.available_balance - turnover;
        
        // let position:&mut Position = self.get_position(&order.code);
        // 计算新成本价（考虑浮点精度）
        // let total_cost = position.volume as f64 * position.cost_price + turnover;
        // let total_volume = position.volume + order.volume;
        // position.cost_price =   total_cost / total_volume as f64;
        // 更新持仓
        // position.volume = total_volume;
        // position.available_vol += order.volume; // T+1 市场需移除这行
                
        // 先处理position，提取需要的数据
        let (total_volume, cost_price) = {
            let position = self.get_position(order.code.clone());
            // 买入成交后，花费总资金
            let total_cost = position.volume as f64 * position.cost_price + turnover;
            // 更新持仓，买入成交后，持仓数量
            position.volume = position.volume + order.volume;                        
            // 计算新成本价（考虑浮点精度）
            position.cost_price = total_cost / position.volume as f64;
            (position.volume, position.cost_price)
        };

        // 记录交易
        self.transactions.push(Transaction {
            time: order.time,
            price: order.price,
            volume: order.volume,
            order_type: order.order_type.clone(),
            remain_vol: total_volume,
            remain_cost: cost_price,
        });

        true
    }

    /// 卖出操作
    pub fn sell(&mut self, order: &Order) -> bool {
        let position = match self.hold.get_mut(&order.code) {
            Some(p) => p,
            None => return false, // 无持仓
        };

        // 可卖数量检查
        if order.volume <= 0 || order.volume > position.available_vol {
            return false;
        }

        // 计算成交金额
        let turnover = order.price * order.volume as f64;

        // 更新资产
        self.available_balance = self.available_balance + turnover;

        // 计算新成本价（当完全卖出时重置为0）
        let total_volume = position.volume - order.volume;
        position.cost_price = if total_volume != 0 {
            (position.volume as f64 * position.cost_price - turnover) / total_volume as f64
        } else {
            0.0
        };

        // 更新持仓
        position.volume = total_volume;
        position.available_vol = position.available_vol - order.volume;

        // 记录交易
        self.transactions.push(Transaction {
            time: order.time,
            price: order.price,
            volume: -order.volume, // 用负数表示卖出
            order_type: order.order_type.clone(),
            remain_vol: total_volume,
            remain_cost: position.cost_price,
        });

        true
    }

    /// 行情变化时，更新持仓市值 todo 
    pub fn on_price_change(&mut self, code: &str, price: f64) {
        if let Some(position) = self.hold.get_mut(&StockCode::from(code)) {
            let after_value = price * position.volume  as f64;
            self.portfolio_value = self.portfolio_value + after_value - position.market_value;
            position.current_price = price;
            position.market_value = after_value;
            self.balance = self.available_balance + self.portfolio_value + self.freeze_balance;
        }
    }
    
    /// 撤单操作（示例实现）
    pub fn cancel_order(&mut self) -> Option<Transaction> {
        // 实际实现需要订单ID管理和状态追踪
        None
    }

    /// 获取持仓信息
    pub fn get_position(&mut self, code: StockCode) -> &mut Position {
        match self.hold.entry(code.clone()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(Position {
                code,
                name: "".to_string(),
                ..Default::default()
            }),
        }
    }
}



/// 持仓信息
#[derive(Debug, Default)]
pub struct Position {
    /// 股票代码
    pub code: StockCode,
    /// 股票名称
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
    /// 市值
    pub market_value: f64,
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
    pub code: StockCode,
    /// 委托时间
    pub time: i64,
    /// 委托价格
    pub price: f64,
    /// 委托数量
    pub volume: i32,
    /// 委托类型 B S
    pub order_type: char,
}

#[derive(Debug,Default,Eq,PartialEq,Ord,PartialOrd,Hash,Clone)]
pub struct StockCode([u8; 8]);

impl From<&str> for StockCode {
    fn from(value: &str) -> Self {
        let mut bytes = [0u8; 8];
        let len = value.len();
        bytes[..len].copy_from_slice(value.as_bytes());
        StockCode(bytes)
    }
}