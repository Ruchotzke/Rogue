use std::ops;

/// A simple vector2int implementation
pub struct Vec2 {
    pub x:i32,
    pub y:i32
}

impl Vec2{

    /// Construct a vec2 of all zeros
    pub fn zero() -> Vec2 {
        Vec2{x:0,y:0}
    }

    /// Construct a vec2 of all ones
    pub fn one() -> Vec2 {
        Vec2{x:1,y:1}
    }

    /// Construct a new vec2
    pub fn new(x:i32,y:i32) -> Vec2 {
        Vec2{x,y}
    }

    /// Scale the vector by a given amount, rounding down.
    pub fn scale(&self, a: f64) -> Vec2 {
        Vec2{
            x: (self.x as f64 * a) as i32,
            y: (self.y as f64 * a) as i32
        }
    }

    /// Compute the magnitude of this vector
    pub fn magnitude(&self) -> f64 {
        (self.x as f64 * self.x as f64 + self.y as f64 * self.y as f64).sqrt()
    }

    /// Compute the continuous distance between two vectors
    pub fn distance(a: &Vec2, b: &Vec2) -> f64 {
        ((a.x - b.x)**2 + (a.y - b.y)**2).sqrt()
    }

}

impl ops::Add<Vec2> for Vec2 {
    type Output = (Vec2);

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = (Vec2);

    fn sub(self, rhs: Vec2) -> Self::Output {
        self + (-rhs)
    }
}
