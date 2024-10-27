use crate::utils::vec2::Vec2;

pub struct Player {
    pub symbol: char,
    pub pos: Vec2,
}

impl Player {
    pub fn new(symbol: char, pos: Vec2) -> Player {
        Player{symbol, pos}
    }
}