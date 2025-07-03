use crate::account::{Account, Order, Position, Transaction,StockCode};
use crate::model::{KLine};
use chrono::{TimeZone, Utc, Duration};

/// 一个低位区间做T策略
#[derive(Debug, Default)]
pub struct KStrategy {

    /// 初始建仓类型  0:数量 1:比例
    init_position_type: u8,
    /// 初始建仓数量
    init_position_volume: i32,
    /// 初始建仓比例
    init_position_proportion:i32,
    
    // 策略参数
    /// 买入价格区间 最低
    buy_price_low: f64,
    /// 买入价格区间 最高
    buy_price_high: f64,
    /// 初始底仓数量
    init_base_volume: i32,
    /// 买入次数影响底仓
    buy_effect_base_volume: u8,
    /// 买入次数，根据买入次数影响底仓
    buy_times: i32,
    ///
    add_volume_every_buy: i32,
    /// 动态底仓数量
    dynamic_base_volume: i32,
    
    
    /// 回调补仓百分比
    add_pos_drawdown_pct: f64,
    /// 初始时，做T区间 止盈 stop_profit
    init_stop_profit:f64,
    /// 动态止盈 stop_profit
    dynamic_stop_profit: f64,
    
    /// 清仓价格，达到清仓价格时清仓
    liquidation_price: f64,
    /// 清仓百分比，达到清仓百分比时清仓
    

    /// 上一个 k线时间
    last_bar_time: i64,
    
}

impl KStrategy {
    pub fn new(buy_price_low: f64, buy_price_high:f64, init_base_volume: i32, add_pos_drawdown_pct: f64, init_stop_profit: f64, liquidation_price:f64) -> Self {
        Self {
            init_position_volume: init_base_volume,
            last_bar_time: 0,
            buy_times: 0,
            buy_price_low,
            buy_price_high,
            init_base_volume,
            add_pos_drawdown_pct,
            dynamic_stop_profit: init_stop_profit,
            add_volume_every_buy: init_base_volume,
            init_stop_profit,
            liquidation_price,
            ..Default::default()
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
        let position = account.get_position(StockCode::from(code));
        // Check new trading day, update available volume
        if bar.time - self.last_bar_time > 12 * 60 * 60 {
            position.available_vol = position.volume;
        }
        position.volume
    }


    /// 初始化建仓
    fn initial_entry(&mut self, bar: &KLine, code: &str, account: &mut Account) {
        let price = bar.close;
        if (self.buy_price_low..=self.buy_price_high).contains(&price) {
            let order = Order {
                market_type: "0".parse().unwrap(),
                code: StockCode::from(code),
                time: bar.time,
                order_type: "B".parse().unwrap(),
                price,
                volume: self.init_position_volume,
            };

            if account.buy(&order) {
                self.dynamic_base_volume = self.init_base_volume;
            }
        }
    }

    fn check_reentry(&mut self, bar: &KLine, code: &str, account: &mut Account) {
        let price = bar.close;
        if let Some(position) = account.hold.get(&StockCode::from(code)) {
            if price <= position.cost_price * (1.0 - self.add_pos_drawdown_pct) {
                let buy_volume = position.volume * 2;
                let order = Order {
                    market_type: ' ',
                    code: StockCode::from(code),
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
        let (sellable, cost_price) = match account.hold.get(&StockCode::from(code)) {
            Some(p) => (p.available_vol, p.cost_price),
            None => return,
        };
        // 触发清仓
        if price > self.liquidation_price {
            // 需要重新获取可变引用进行卖出操作
            let order = Order {
                market_type: ' ',
                code: StockCode::from(code),
                time: bar.time,
                order_type: "S".parse().unwrap(),
                price,
                volume: sellable,
            };

            if account.sell(&order) {
                // todo 清仓成功后 更新 base_position ???
                self.dynamic_stop_profit = self.init_stop_profit;
                // self.dynamic_base_volume = self.init_base_volume;
                // self.buy_times = 0;
            }
        } else if price >= cost_price + self.dynamic_stop_profit + (0.02 * self.buy_times as f64) {
            if sellable > self.dynamic_base_volume {
                let sell_volume = sellable - self.dynamic_base_volume - (self.buy_times * self.add_volume_every_buy);

                // 需要重新获取可变引用进行卖出操作
                let order = Order {
                    market_type: ' ',
                    code: StockCode::from(code),
                    time: bar.time,
                    order_type: "S".parse().unwrap(),
                    price,
                    volume: sell_volume,
                };

                if account.sell(&order) {
                    if let Some(position) = account.hold.get(&StockCode::from(code)) {
                        // 
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


