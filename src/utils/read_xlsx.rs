use calamine::{Reader, Xlsx};

use tracing::info;
use crate::model::game::Game;
use crate::model::game::PanType::{Baidu, Other, Quark, XunLei};

pub fn load_xlsx() -> Vec<Game> {
    info!("开始加载xlsx中的游戏列表");
    let xlsx_bytes = include_bytes!("../game.xlsx");
    let cursor = std::io::Cursor::new(xlsx_bytes);
    let mut workbook = Xlsx::new(cursor).expect("无法从内存加载Xlsx文件");
    let mut id = 0;
    let mut game_list: Vec<Game> = vec![];
    if let Ok(range) = workbook.worksheet_range("自定义词库备份游戏"){
        for row in range.rows(){
            if id == 0 {
                id += 1;
                continue;
            }
            let cell_id = id;
            let cell_name = row[1].to_string();
            let cell_url = row[2].to_string();
            let cell_type = match row[2].to_string().as_str() {
                s if s.contains("xunlei") => XunLei,
                s if s.contains("baidu") => Baidu,
                s if s.contains("quark") => Quark,
                _ => Other
            };
            game_list.push(
                Game::new(
                    cell_id,
                    cell_name,
                    cell_url,
                    cell_type
                )
            );
            id += 1;
        }
        info!("成功加载 {} 个游戏", game_list.len());
        game_list
    } else {
        panic!("无法读取Xlsx文件");
    }
}