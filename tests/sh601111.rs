use backtest::account::{Account, StockCode};
use backtest::model::{ KLine};
use backtest::strategy::k_strategy::KStrategy;
use csv::Reader;


#[test]
fn 区间做t() {
    
    // 1. 读取 JSON 文件
    // let file = File::open(r"E:\OneDrive\1股票交易\11-回测\USHA601111-day2.json").unwrap();

    // 2. 解析 JSON 数据
    // let mut bars: Vec<KLine> = serde_json::from_reader(file).unwrap();
    let mut bars = Reader::from_path(r"A:\data\day\USHA601111.csv").unwrap();

    // 3. 初始化账户
    let mut account = Account {
        balance: 1_000_000.0,
        available_balance: 1_000_000.0,
        ..Default::default()
    };
    let code = "600795";

    // 4. 创建策略
    // let mut strategy = KStrategy::new(5.9, 7.8,20000,0.05, 0.4, 11.0);
    // let mut strategy = KStrategy::new(5.9, 7.8,50000,0.1, 0.4, 11.0);
    let mut strategy = KStrategy::new(5.9, 7.8,130000,0.1, 0.5, 11.0);

    // 5. 按时间排序
    // bars.sort_by_key(|k| k.time);

    // 6. 处理每个 K 线
    let iter = bars.deserialize();
    // 6. 处理每个 K 线
    for ite in iter {
        let bar: KLine = ite.unwrap();
        strategy.process_bar(&bar, &code, &mut account);
    }
    // 7. 打印结果
    let position = account.hold.get(&StockCode::from(code)).unwrap();

    strategy.print_results(&account.transactions, position, &account);
}
