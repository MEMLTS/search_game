#[derive(Debug)]
pub struct Game {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) pan_type: PanType,
}

#[derive(Debug)]
pub enum PanType {
    Quark,
    Baidu,
    XunLei,
    Other
}

impl Game {
    pub fn new (id: i32, name: String, url: String, pan_type: PanType) -> Self {
        Self {
            id,
            name,
            url,
            pan_type
        }
    }
}