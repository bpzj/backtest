use crate::account::Account;
use crate::model::KLine;
use crate::strategy::k_strategy::KStrategy;
use csv::Reader;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::Arc;
use crate::account::StockCode;

pub struct StrategyApp {
    strategy_params: StrategyParams,
    running: bool,
    balance_points: Vec<[f64; 2]>,
}

struct StrategyParams {
    entry_range: [f64; 2],
    base_volume: i32,
    t_stop_loss_pct: f64,
    t_stop_profit: f64,
    liquidation_price: f64,
}

impl Default for StrategyApp {
    fn default() -> Self {
        Self {
            strategy_params: StrategyParams {
                entry_range: [5.9, 7.9],
                base_volume: 10000,
                t_stop_loss_pct: 0.02,
                t_stop_profit: 0.1,
                liquidation_price: 8.5,
            },
            running: false,
            balance_points: Vec::new(),
        }
    }
}

impl eframe::App for StrategyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("K线策略参数设置");
            
            ui.horizontal(|ui| {
                ui.label("入场价格区间：");
                ui.add(egui::DragValue::new(&mut self.strategy_params.entry_range[0]).speed(0.1));
                ui.label("到");
                ui.add(egui::DragValue::new(&mut self.strategy_params.entry_range[1]).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("底仓数量：");
                ui.add(egui::DragValue::new(&mut self.strategy_params.base_volume));
            });

            ui.horizontal(|ui| {
                ui.label("回调买入百分比：");
                ui.add(egui::DragValue::new(&mut self.strategy_params.t_stop_loss_pct).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("止盈百分比：");
                ui.add(egui::DragValue::new(&mut self.strategy_params.t_stop_profit).speed(0.01));
            });

            ui.horizontal(|ui| {
                ui.label("清仓价格：");
                ui.add(egui::DragValue::new(&mut self.strategy_params.liquidation_price).speed(0.1));
            });

            ui.add_space(10.0);

            if ui.button(if self.running { "停止策略" } else { "运行策略" }).clicked() {
                if !self.running {
                    self.running = true;  // Set running state before execution
                    self.run_strategy();
                    self.running = false;  // Reset state after completion
                } 
            }

            // 显示资金曲线
            if !self.balance_points.is_empty() {
                ui.add_space(20.0);
                Plot::new("资金变化").show(ui, |plot_ui| {
                    let plot_points = PlotPoints::new(self.balance_points.clone());
                    plot_ui.line(Line::new("资金", plot_points));
                });
            }
        });
    }
}

impl StrategyApp {
    fn run_strategy(&mut self) {
        let mut strategy = KStrategy::new(
            self.strategy_params.entry_range[0],
            self.strategy_params.entry_range[1],
            self.strategy_params.base_volume,
            self.strategy_params.t_stop_loss_pct,
            self.strategy_params.t_stop_profit,
            self.strategy_params.liquidation_price,
        );
        
        // TODO: 实现实际的策略运行逻辑
        let mut account = Account {
            balance: 1_000_000.0,
            available_balance: 1_000_000.0,
            ..Default::default()
        }; // 初始资金100万
        let mut bars = Reader::from_path(r"A:\data\day\USHA601111.csv").unwrap();
        let code = "600795";
        // let mut strategy = KStrategy::new([5.9, 7.9],10000,0.02, 0.1, 9.0);
        self.balance_points.clear();
        let mut time_index = 0.0_f64;

        let iter = bars.deserialize();
        for ite in iter {
            let bar: KLine = ite.unwrap();
            strategy.process_bar(&bar, &code, &mut account);
            // todo 计算的金额不对 
            self.balance_points.push([time_index, account.balance]);
            time_index += 1.0;
        }
        // 7. 打印结果
        let position = account.hold.get(&StockCode::from(code)).unwrap();

        strategy.print_results(&account.transactions, position, &account);
    }
}

pub fn run_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 500.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "K线策略控制面板",
        options,
        Box::new(|cc| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "simhei".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!("../../assets/fonts/simhei.ttf"))),
            );
            
            fonts.families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "simhei".to_owned());
                
            cc.egui_ctx.set_fonts(fonts);
            
            Ok(Box::new(StrategyApp::default()))
        }),
    )
} 