mod utils;
mod model;
mod gui;

use eframe::glow::INFO_LOG_LENGTH;
use tracing::info;

use crate::gui::app::App;
use crate::utils::build_index::build_game_index;
use crate::utils::read_xlsx::load_xlsx;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("开始启动程序...");
    info!("正在初始化索引...");
    let game_list = load_xlsx();
    let build_index = build_game_index(game_list)?;
    info!("索引初始化完成!");
    info!("正在启动GUI界面...");

    info!("INFO_LOG_LENGTH: {}", INFO_LOG_LENGTH);
    info!("===========================================================");
    info!("本程序为十一实验室资源搜索工具，请勿用于商业用途，请勿用于非法用途!");
    info!("官网:https://zsy11.com  QQ交流群:519409405");
    info!("本工具完全免费使用,所有数据均在本地,不上传任何数据!");
    info!("===========================================================");

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "资源搜索 - 十一实验室 官网:https://zsy11.com  QQ交流群:519409405",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(App::new(build_index)))
        }),
    ).expect("启动GUI界面失败!!!");
    Ok(())
}
