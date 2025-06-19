use std::collections::HashMap;
use crate::model::{Assets, KLine, Order, Position, Transaction};

// 账户主体
#[derive(Debug,Default)]
pub struct Account {
    pub name: String,
    /// 记录资金、盈亏
    // pub assets: Assets,
    /// 总资产
    pub balance: f64,
    /// 冻结金额
    pub freeze_balance: f64,
    /// 可用金额
    pub available_balance: f64,
    /// 总市值
    pub shi_zhi: f64,          
    /// 总盈亏
    pub profit: f64,            
    
    /// 持仓
    pub hold: HashMap<String, Position>,
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
        let position:&mut Position = self.hold.get_mut(&order.code).unwrap();
        let turnover = order.price * order.volume as f64;

        // 资金检查
        if self.available_balance < turnover {
            return false;
        }

        // 更新资产
        self.available_balance -= turnover;

        // 计算新成本价（考虑浮点精度）
        let total_cost = position.volume as f64 * position.cost_price + turnover;
        let total_volume = position.volume + order.volume;
        position.cost_price =   total_cost / total_volume as f64;

        // 更新持仓
        position.volume = total_volume;
        // position.available_vol += order.volume; // T+1 市场需移除这行

        // 记录交易
        self.transactions.push(Transaction {
            time: order.time,
            price: order.price,
            volume: order.volume,
            order_type: order.order_type.clone(),
            remain_vol: total_volume,
            remain_cost: position.cost_price,
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
        self.available_balance += turnover;

        // 计算新成本价（当完全卖出时重置为0）
        let total_volume = position.volume - order.volume;
        position.cost_price = if total_volume != 0 {
            (position.volume as f64 * position.cost_price - turnover) / total_volume as f64
        } else {
            0.0
        };

        // 更新持仓
        position.volume = total_volume;
        position.available_vol -= order.volume;

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

    /// 行情变化时，更新持仓
    pub fn on_price_change(&mut self, code: &str, price: f64) {
        if let Some(position) = self.hold.get_mut(&code.to_string()) {
            position.current_price = price;
            self.shi_zhi = position.current_price * position.volume as f64;
            self.balance = self.available_balance + self.shi_zhi + self.freeze_balance;
        }
    }
    
    /// 撤单操作（示例实现）
    pub fn cancel_order(&mut self) -> Option<Transaction> {
        // 实际实现需要订单ID管理和状态追踪
        None
    }
}
