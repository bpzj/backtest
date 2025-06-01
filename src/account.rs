use std::collections::HashMap;
use crate::model::{Assets, Order, Position, Transaction};

// 账户主体
#[derive(Debug,Default)]
pub struct Account {
    pub name: String,
    pub assets: Assets,
    pub positions: HashMap<String, Position>,
    pub transactions: Vec<Transaction>,
    pub orders: Vec<Order>,
}

impl Account {
    /// 买入操作
    pub fn buy(&mut self, order: &Order) -> bool {
        let position = self
            .positions
            .entry(order.code.clone())
            .or_insert_with(|| Position {
                code: "".to_string(),
                name: "".to_string(),
                volume: 0,
                cost_price: 0.0,
                available_vol: 0,
                current_price: 0.0,
            });

        let turnover = order.price * order.volume as f64;

        // 资金检查
        if self.assets.available_balance < turnover {
            return false;
        }

        // 更新资产
        self.assets.available_balance -= turnover;

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
        let position = match self.positions.get_mut(&order.code) {
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
        self.assets.available_balance += turnover;

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

    /// 撤单操作（示例实现）
    pub fn cancel_order(&mut self) -> Option<Transaction> {
        // 实际实现需要订单ID管理和状态追踪
        None
    }
}
