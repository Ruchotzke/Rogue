use super::cell::{Cell, CellAccess};
use std::vec::Vec;
use crate::world::cell;
use crate::world::world_gen::room::Room;

pub struct Map{
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let cells = vec![Cell{access: CellAccess::CLOSED}; (width * height) as usize];
        Map {
            width, height, cells
        }
    }

    /// Get a reference to the cell at a given position
    /// Performs a bounds check and may fail
    pub fn get_cell(&self, x: u32, y: u32) -> Result<&mut Cell, String> {
        /* Bounds check! */
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return Err(format!("x:{} y:{} out of map range", x, y));
        }

        return Ok(&mut self.cells[y as usize * self.width as usize + x as usize])
    }

    /// Clear out a room on this map
    pub fn add_room(&mut self, room: &Room) {
        let bounds = room.get_bounds();

        for x in bounds.0.x..bounds.1.x {
            for y in bounds.0.y..bounds.1.y {
                self.get_cell(x as u32, y as u32).unwrap().access = CellAccess::OPEN;
            }
        }
    }
}