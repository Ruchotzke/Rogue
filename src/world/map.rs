use std::cmp::max;
use super::cell::{Cell, CellAccess};
use std::vec::Vec;
use crate::utils::vec2::Vec2;
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
    pub fn get_cell(&mut self, x: u32, y: u32) -> Result<&mut Cell, String> {
        /* Bounds check! */
        if x >= self.width || y >= self.height {
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

    /// Add a hallway between two rooms
    pub fn add_hall(&mut self, a: &Room, b: &Room) {
        /* Pick a random point from both rooms */
        let a = a.get_rand_point(&mut rand::thread_rng());
        let b = b.get_rand_point(&mut rand::thread_rng());

        /* Connect them */
        self.tunnel_right_angle(&a, &b);
    }

    /// Tunnel a line between two positions
    /// Does not draw a straight line, but goes up and over
    pub fn tunnel_right_angle(&mut self, a: &Vec2, b: &Vec2){
        /* Get the starting point, which is the leftmost position */
        let s = match a.x < b.x {
            true => {a}
            false => {b}
        };

        /* Compute the rightmost point for the horizontal */
        let h = Vec2::new(max(a.x, b.x), s.y);

        /* Compute the endpoint */
        let e = match a.x < b.x {
            true => {b}
            false => {a}
        };

        /* Tunnel! */
        for x in (s.x..=h.x) {
            self.get_cell(x as u32, s.y as u32).unwrap().access = CellAccess::OPEN;
        }

        if(h.y < e.y){
            for y in h.y..=e.y {
                self.get_cell(h.x as u32, y as u32).unwrap().access = CellAccess::OPEN;
            }
        }
        else {
            for y in e.y..=h.y {
                self.get_cell(h.x as u32, y as u32).unwrap().access = CellAccess::OPEN;
            }
        }
    }
}