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

    Ok(())
}
