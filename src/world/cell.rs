
/// A single point on a map.
pub struct Cell{
    pub access: CellAccess
}

pub enum CellAccess {
    OPEN,
    CLOSED
}

impl Cell {
    pub fn new() -> Cell{
        Cell{access: CellAccess::OPEN}
    }
}