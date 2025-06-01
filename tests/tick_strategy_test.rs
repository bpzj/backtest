use backtest::account::Account;
use backtest::model::{Assets, TickData};
use backtest::strategy::tick_strategy::TickStrategy;

#[test]
fn test_tick_strategy() {
    // 1. 初始化账户
    let mut account = Account {
        assets: Assets {
            balance: 1_000_000.0,
            freeze_balance: 0.0,
            available_balance: 1_000_000.0,
            shi_zhi: 0.0,
            ying_kui: 0.0,
        },
        ..Default::default()
    };
    
    let code = "000001".to_string();
    
    // 2. 创建策略实例
    // 参数说明：
    // - min_spread: 最小价差 0.01
    // - min_volume: 最小交易量 1000
    // - trade_volume: 单次交易量 2000
    // - stop_loss_points: 止损点数 0.03
    // - take_profit_points: 止盈点数 0.05
    // - cooldown_period: 冷却时间 3秒
    let mut strategy = TickStrategy::new(0.01, 1000, 2000, 0.03, 0.05, 3);
    
    // 3. 模拟tick数据
    let tick = TickData {
        time: 1625097600, // 2021-07-01 00:00:00
        last_price: 10.0,
        volume: 100000,
        
        bid1_price: 9.99,
        bid1_volume: 2000,
        bid2_price: 9.98,
        bid2_volume: 3000,
        bid3_price: 9.97,
        bid3_volume: 4000,
        bid4_price: 9.96,
        bid4_volume: 5000,
        bid5_price: 9.95,
        bid5_volume: 6000,
        
        ask1_price: 10.01,
        ask1_volume: 1500,
        ask2_price: 10.02,
        ask2_volume: 2500,
        ask3_price: 10.03,
        ask3_volume: 3500,
        ask4_price: 10.04,
        ask4_volume: 4500,
        ask5_price: 10.05,
        ask5_volume: 5500,
    };
    
    // 4. 处理tick数据
    strategy.process_tick(&tick, &code, &mut account);
    
    // 5. 获取结果
    if let Some(position) = account.positions.get(&code) {
        println!("\n持仓信息：");
        println!("持仓数量：{}股", position.volume);
        println!("持仓成本：{:.3}", position.cost_price);
        println!("可用数量：{}股", position.available_vol);
    }
    
    println!("\n账户信息：");
    println!("可用资金：{:.3}", account.assets.available_balance);
    println!("总资产：{:.3}", account.assets.balance);
} 