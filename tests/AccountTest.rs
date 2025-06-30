
use backtest::account::{Account, Order, StockCode};
use backtest::model::{KLine};
use csv::Reader;

#[test]
fn test_account() {
    //初始化账户
    let mut account = Account {
        balance: 1_000_000.0,
        available_balance: 1_000_000.0,
        ..Default::default()
    };
    let code = "600795";

    // 模拟买入一单
    let order = Order{
        market_type: "0".parse().unwrap(),
        code: StockCode::from(code),
        time: 1,
        order_type: "B".parse().unwrap(),
        price: 1.0,
        volume: 100,
    };
    account.buy(&order);
    account.on_price_change(code, 1.0);

    // 账户市值
    assert_eq!(account.balance, 1_000_000.0);
    assert_eq!(account.available_balance, 1_000_000.0 - 100.0);
    assert_eq!(account.portfolio_value, 100.0);                  // 总市值
    assert_eq!(account.profit, 0.0);

    // 持仓
    let pos = account.get_position(StockCode::from(code));
    assert_eq!(pos.volume, 100);          // 持仓量
    assert_eq!(pos.current_price, 1.0);   // 当前价
    assert_eq!(pos.market_value, 100.0);  // 市值
    assert_eq!(pos.cost_price, 1.0);    // 成本价


    // 再买一单
    let code2 = "601111";
    let order2 = Order{
        market_type: "0".parse().unwrap(),
        code: StockCode::from(code2),
        time: 1,
        order_type: "B".parse().unwrap(),
        price: 1.0,
        volume: 200,
    };
    account.buy(&order2);
    account.on_price_change(code2, 1.0);

    assert_eq!(account.balance, 1_000_000.0);
    assert_eq!(account.available_balance, 1_000_000.0 - 300.0);
    assert_eq!(account.portfolio_value, 300.0);                  // 总市值
    assert_eq!(account.profit, 0.0);

    // 持仓
    let pos2 = account.get_position(StockCode::from(code2));
    assert_eq!(pos2.volume, 200);          // 持仓量
    assert_eq!(pos2.current_price, 1.0);   // 当前价
    assert_eq!(pos2.market_value, 200.0);  // 市值
    assert_eq!(pos2.cost_price, 1.0);      // 成本价
}
