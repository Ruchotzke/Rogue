
/// A single point on a map.
pub struct Cell{
    pub access: CellAccess
}

#[derive(Copy, Clone)]
pub enum CellAccess {
    OPEN,
    CLOSED
}

impl Cell {
    pub fn new() -> Cell{
        Cell{access: CellAccess::OPEN}
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell{access: self.access}
    }
}