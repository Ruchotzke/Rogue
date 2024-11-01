use crate::utils::vec2f::Vec2f;

pub struct Circle {
    center: Vec2f,
    radius: f64
}

impl Circle {
    pub fn new(center: Vec2f, radius: f64) -> Circle {
        Circle { center, radius }
    }

    pub fn in_circle(&self, point: Vec2f) -> bool {
        Vec2f::dist(&self.center, &point) <= self.radius
    }
}