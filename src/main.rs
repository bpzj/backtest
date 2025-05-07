mod model;
mod account;
mod strategy;

use crate::account::Account;
use crate::model::{Assets, KLine};
use crate::strategy::KStrategy;
use serde_json;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. 读取 JSON 文件
    let file = File::open(r"D:\OneDrive\1股票交易\11回测\600795-5min.json")?;

    // 2. 解析 JSON 数据
    let mut bars: Vec<KLine> = serde_json::from_reader(file)?;

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
    let mut strategy = KStrategy::new();

    // 5. 按时间排序
    bars.sort_by_key(|k| k.time);

    // 6. 处理每个 K 线
    for bar in &bars {
        strategy.process_bar(bar, &code, &mut account);
    }

    // 7. 打印结果
    let position = account.positions.get(&code).unwrap();

    strategy.print_results(&account.transactions, position, &account.assets);

    Ok(())
}
