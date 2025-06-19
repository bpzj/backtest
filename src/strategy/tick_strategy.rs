use crate::account::Account;
use crate::model::{Order, TickData};

/// Tick级别策略 - 基于盘口信息的简单策略
pub struct TickStrategy {
    // 策略参数
    /// 最小价差
    min_spread: f64,
    /// 最小交易量
    min_volume: i32,
    /// 单次交易量
    trade_volume: i32,
    /// 止损点数
    stop_loss_points: f64,
    /// 止盈点数
    take_profit_points: f64,
    
    // 状态变量
    last_trade_price: f64,
    last_trade_time: i64,
    position: i32,
    
    // 冷却时间(秒)
    cooldown_period: i64,
}

impl TickStrategy {
    pub fn new(
        min_spread: f64,
        min_volume: i32,
        trade_volume: i32,
        stop_loss_points: f64,
        take_profit_points: f64,
        cooldown_period: i64,
    ) -> Self {
        Self {
            min_spread,
            min_volume,
            trade_volume,
            stop_loss_points,
            take_profit_points,
            cooldown_period,
            last_trade_price: 0.0,
            last_trade_time: 0,
            position: 0,
        }
    }

    pub fn process_tick(&mut self, tick: &TickData, code: &str, account: &mut Account) {
        // 检查冷却期
        if tick.time - self.last_trade_time < self.cooldown_period {
            return;
        }

        // 计算买卖盘价差
        let spread = tick.ask1_price - tick.bid1_price;
        
        // 检查是否有足够的盘口深度
        if tick.bid1_volume < self.min_volume || tick.ask1_volume < self.min_volume {
            return;
        }

        // 持仓为0时的开仓逻辑
        if self.position == 0 {
            // 当价差小于最小价差时可能存在套利机会
            if spread <= self.min_spread {
                // 偏向做多：当买一价格高于最新成交价，且卖一挂单量大于最小交易量
                if tick.bid1_price > tick.last_price && tick.ask1_volume > self.min_volume {
                    self.open_long(tick, code, account);
                }
                // 偏向做空：当卖一价格低于最新成交价，且买一挂单量大于最小交易量
                else if tick.ask1_price < tick.last_price && tick.bid1_volume > self.min_volume {
                    self.open_short(tick, code, account);
                }
            }
        } else {
            // 持仓不为0时检查是否需要平仓
            self.check_close_position(tick, code, account);
        }
    }

    fn open_long(&mut self, tick: &TickData, code: &str, account: &mut Account) {
        let order = Order {
            market_type: ' ',
            code: code.to_string(),
            time: tick.time,
            order_type: 'B',
            price: tick.ask1_price,
            volume: self.trade_volume,
        };

        if account.buy(&order) {
            self.position = self.trade_volume;
            self.last_trade_price = tick.ask1_price;
            self.last_trade_time = tick.time;
        }
    }

    fn open_short(&mut self, tick: &TickData, code: &str, account: &mut Account) {
        let order = Order {
            market_type: ' ',
            code: code.to_string(),
            time: tick.time,
            order_type: 'B',
            price: tick.bid1_price,
            volume: self.trade_volume,
        };

        if account.buy(&order) {
            self.position = -self.trade_volume;
            self.last_trade_price = tick.bid1_price;
            self.last_trade_time = tick.time;
        }
    }

    fn check_close_position(&mut self, tick: &TickData, code: &str, account: &mut Account) {
        if self.position > 0 {
            // 多仓止损
            if tick.bid1_price <= self.last_trade_price - self.stop_loss_points {
                self.close_position(tick, code, account);
            }
            // 多仓止盈
            else if tick.bid1_price >= self.last_trade_price + self.take_profit_points {
                self.close_position(tick, code, account);
            }
        } else if self.position < 0 {
            // 空仓止损
            if tick.ask1_price >= self.last_trade_price + self.stop_loss_points {
                self.close_position(tick, code, account);
            }
            // 空仓止盈
            else if tick.ask1_price <= self.last_trade_price - self.take_profit_points {
                self.close_position(tick, code, account);
            }
        }
    }

    fn close_position(&mut self, tick: &TickData, code: &str, account: &mut Account) {
        let order = Order {
            market_type: ' ',
            code: code.to_string(),
            time: tick.time,
            order_type: 'S',
            price: if self.position > 0 { tick.bid1_price } else { tick.ask1_price },
            volume: self.position.abs(),
        };

        if account.sell(&order) {
            self.position = 0;
            self.last_trade_time = tick.time;
        }
    }
} 