use backtest::account::Account;
use backtest::model::{Assets, KLine};
use backtest::strategy::k_strategy::KStrategy;
use csv::Reader;

#[test]
fn 区间做t() {
    // todo 需要先转成前复权数据
    
    // 1. 读取 JSON 文件
    // let file = File::open(r"A:\A\1day\USHA601111.csv").unwrap();

    // 2. 解析 JSON 数据
    // let mut bars: Vec<KLine> = serde_json::from_reader(file).unwrap();
    let mut bars = Reader::from_path(r"A:\A\1day\USHA600795.csv").unwrap();
    
    // 3. 初始化账户
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
    let code = "600795".to_string();

    // 4. 创建策略
    let mut strategy = KStrategy::new([4.1, 4.46],2000,0.02,0.04,6.0);

    // 5. 按时间排序
    // bars.sort_by_key(|k| k.time);

    let iter = bars.deserialize();
    // 6. 处理每个 K 线
    for ite in iter {
        let bar: KLine = ite.unwrap();
        strategy.process_bar(&bar, &code, &mut account);
    }

    // 7. 打印结果
    let position = account.positions.get(&code).unwrap();

    strategy.print_results(&account.transactions, position, &account.assets);
}
