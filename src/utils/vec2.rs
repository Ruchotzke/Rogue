/// A simple vector2int implementation
pub struct Vec2 {
    pub x:i32,
    pub y:i32
}

impl Vec2{
    pub fn zero() -> Vec2 {
        Vec2{x:0,y:0}
    }

    pub fn new(x:i32,y:i32) -> Vec2 {
        Vec2{x,y}
    }

}
