mod utils;
mod model;
mod gui;

use crate::gui::app::App;
use crate::utils::build_index::build_game_index;
use crate::utils::read_xlsx::load_xlsx;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    let game_list = load_xlsx();
    let build_index = build_game_index(game_list)?;

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
