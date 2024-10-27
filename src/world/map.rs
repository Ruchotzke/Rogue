use super::cell::Cell;
use std::vec::Vec;
use crate::world::cell;

pub struct Map{
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let cells = vec![cell::Cell::new(); (width * height) as usize];
        Map {
            width, height, cells
        }
    }
}