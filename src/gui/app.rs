use std::sync::Arc;
use eframe::egui::{self, Context};
use eframe::Frame;
use tantivy::Index;
use crate::model::game::Game;
use crate::utils::search_game::search_game;

pub struct App {
    keyword: String,
    result: Vec<Game>,
    index: Index
}

fn setup_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = egui::vec2(8.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(12.0 as i8);
    // 加载中文字体
    use egui::FontDefinitions;
    let mut fonts = FontDefinitions::default();

    // 将中文字体加入到所有文本
    fonts.font_data.insert(
        "阿里妈妈方圆体 VF".to_owned(),
        Arc::from(egui::FontData::from_static(include_bytes!("../../AlimamaFangYuanTiVF-Thin.ttf"))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "阿里妈妈方圆体 VF".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "阿里妈妈方圆体 VF".to_owned());

    ctx.set_fonts(fonts);
    ctx.set_style(style);
}
impl App {
    pub(crate) fn new (index: Index) -> Self {
        App {
            keyword: "".to_string(),
            result: vec![],
            index
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        setup_style(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // ===== 搜索栏 =====
            ui.horizontal(|ui| {
                ui.add_sized(
                    egui::vec2(ui.available_width() - 90.0, 32.0),
                    egui::TextEdit::singleline(&mut self.keyword)
                        .hint_text("输入游戏名 / 关键词"),
                );

                let search_clicked = ui
                    .add_sized(
                        egui::vec2(80.0, 32.0),
                        egui::Button::new("搜索"),
                    )
                    .clicked();

                // 回车 or 点击按钮都能搜索
                if search_clicked || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.result = search_game(&self.index, &self.keyword).unwrap();
                }
            });

            ui.add_space(12.0);

            // ===== 搜索结果 =====
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.result.is_empty() {
                    ui.label(
                        egui::RichText::new("暂无结果")
                            .color(egui::Color32::GRAY),
                    );
                }

                for game in &self.result {
                    egui::Frame::group(ui.style())
                        .inner_margin(egui::Margin::same(8.0 as i8))
                        .show(ui, |ui| {
                            // 标题 + 类型
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&game.name)
                                        .strong()
                                        .size(16.0),
                                );

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        badge(
                                            ui,
                                            format!("{:?}", game.pan_type).as_str(),
                                        );
                                    },
                                );
                            });

                            ui.add_space(6.0);

                            // URL
                            ui.label(
                                egui::RichText::new(&game.url)
                                    .strong()
                                    .size(15.0)
                            );
                        });

                    ui.add_space(8.0);
                }
            });
        });
    }
}

fn badge(ui: &mut egui::Ui, text: &str) {
    egui::Frame::none()
        .fill(egui::Color32::DARK_GRAY)
        .rounding(egui::Rounding::same(6.0 as u8))
        .inner_margin(egui::Margin::symmetric(8.0 as i8, 4.0 as i8))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .small()
                    .color(egui::Color32::WHITE),
            );
        });
}
