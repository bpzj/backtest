use crate::account::Account;
use crate::model::{KLine, Order, Position, Transaction};
use chrono::{TimeZone, Utc, Duration};

/// 一个低位区间做T策略
pub struct KStrategy {
    // 初始持仓
    base_position: i32,
    
    // 策略参数
    /// 买入次数，根据买入次数影响底仓
    buy_times: i32,
    /// 底仓
    base_volume: i32,
    /// 低价做 T 的价格区间
    entry_range: [f64; 2],
    
    /// 回调买入百分比
    t_stop_loss_pct: f64,
    /// 初始时，做T区间 止盈 stop_profit
    init_t_stop_profit:f64,
    /// 做T区间 止盈 stop_profit
    t_stop_profit: f64,
    
    /// 清仓价格，达到清仓价格时清仓
    liquidation_price: f64,
    /// 清仓百分比，达到清仓百分比时清仓
    
    /// 上一个 k线时间
    last_bar_time: i64,
    
}

impl KStrategy {
    pub fn new(entry_range: [f64; 2], base_volume: i32, t_stop_loss_pct: f64, t_stop_profit: f64, liquidation_price:f64) -> Self {
        Self {
            base_position: 0,
            last_bar_time: 0,
            buy_times: 0,
            entry_range,
            base_volume,
            t_stop_loss_pct,
            t_stop_profit,
            init_t_stop_profit:t_stop_profit,
            liquidation_price
        }
    }

    pub fn process_bar(&mut self, bar: &KLine, code: &str, account: &mut Account) {
        let volume = self.get_vol(bar, code, account);

        if volume == 0 {
            self.initial_entry(bar, code, account);
        } else {
            self.check_reentry(bar, code, account);
            self.check_profit(bar, code, account);
        }

        // Update final values
        // if let Some(position) = account.positions.get_mut(code) {
        account.on_price_change(code, bar.close);
            
        // }
        self.last_bar_time = bar.time;
    }

    fn get_vol(&mut self, bar: &KLine, code: &str, account: &mut Account) -> i32 {
        // Get position volume in a separate scope
        let position = account.get_position(code);
        // Check new trading day, update available volume
        if bar.time - self.last_bar_time > 12 * 60 * 60 {
            position.available_vol = position.volume;
        }
        position.volume
    }

    fn initial_entry(&mut self, bar: &KLine, code: &str, account: &mut Account) {
        let price = bar.close;
        if (self.entry_range[0]..=self.entry_range[1]).contains(&price) {
            let order = Order {
                market_type: ' ',
                code: code.to_string(),
                time: bar.time,
                order_type: "B".parse().unwrap(),
                price,
                volume: self.base_volume,
            };

            if account.buy(&order) {
                self.base_position = self.base_volume;
            }
        }
    }

    fn check_reentry(&mut self, bar: &KLine, code: &str, account: &mut Account) {
        let price = bar.close;
        if let Some(position) = account.hold.get(code) {
            if price <= position.cost_price * (1.0 - self.t_stop_loss_pct) {
                let buy_volume = position.volume * 2;
                let order = Order {
                    market_type: ' ',
                    code: code.to_string(),
                    time: bar.time,
                    order_type: "B".parse().unwrap(),
                    price,
                    volume: buy_volume,
                };

                if account.buy(&order) {
                    self.buy_times += 1;
                }
            }
        }
   
   
    }

    fn check_profit(&mut self, bar: &KLine, code: &str, account: &mut Account) {
        let price = bar.close;
        // 先获取持仓数据（不持有引用）
        let (sellable, cost_price) = match account.hold.get(code) {
            Some(p) => (p.available_vol, p.cost_price),
            None => return,
        };
        // 触发清仓
        if price > self.liquidation_price {
            // 需要重新获取可变引用进行卖出操作
            let order = Order {
                market_type: ' ',
                code: code.to_string(),
                time: bar.time,
                order_type: "S".parse().unwrap(),
                price,
                volume: sellable,
            };

            if account.sell(&order) {
                // 卖出成功后更新 base_position ???
                self.t_stop_profit = self.init_t_stop_profit
            }
        } else if price >= cost_price + self.t_stop_profit + (0.02 * self.buy_times as f64) {
            if sellable > self.base_position {
                let sell_volume = sellable - self.base_position - (self.buy_times * self.base_volume);

                // 需要重新获取可变引用进行卖出操作
                let order = Order {
                    market_type: ' ',
                    code: code.to_string(),
                    time: bar.time,
                    order_type: "S".parse().unwrap(),
                    price,
                    volume: sell_volume,
                };

                if account.sell(&order) {
                    // 卖出成功后更新 base_position
                    if let Some(position) = account.hold.get(code) {
                        self.base_position = position.volume;
                    }
                }
            }
        };
    }

    pub fn print_results(&self, transactions: &[Transaction], position: &Position, account: &Account) {
        println!("\n交易记录：");
        for t in transactions {
            // 将时间戳转换为UTC时间
            let utc_time = Utc.timestamp_opt(t.time, 0).single().unwrap();
            // 手动调整为UTC+8时区（直接加8小时）
            let utc8_time = utc_time + Duration::hours(8);
            
            println!(
                "{} - {:4} {}股 @ {:.2}  成交后{}股 成交后成本{:.3}",
                utc8_time.format("%Y-%m-%d"), t.order_type, t.volume, t.price, t.remain_vol, t.remain_cost
            );
        }

        println!("\n最终持仓：{}股", position.volume);
        println!("持仓成本：{:.3}", position.cost_price);
        println!("剩余现金：{:.3}", account.available_balance);
        println!("总资产：{:.3}", account.balance);
    }
}


