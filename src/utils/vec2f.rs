use std::ops;

pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vec2f {
    pub fn new(x: f64, y: f64) -> Vec2f {
        Vec2f{x,y}
    }

    pub fn zero() -> Vec2f {
        Vec2f{x: 0.0, y: 0.0}
    }

    pub fn one() -> Vec2f {
        Vec2f{x: 1.0, y: 1.0}
    }

    /// Distance between two vectors
    pub fn dist(a: &Vec2f, b: &Vec2f) -> f64 {
        ((a.x - b.x)*(a.x - b.x) + (a.y - b.y)*(a.y - b.y)).sqrt()
    }
}

impl ops::Add for Vec2f {
    type Output = (Vec2f);

    fn add(self, rhs: Self) -> Self::Output {
        Vec2f{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl ops::Sub for Vec2f {
    type Output = (Vec2f);

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2f{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl ops::Mul<f64> for Vec2f {
    type Output = (Vec2f);

    /// Scale the vector by another
    fn mul(self, rhs: f64) -> Self::Output {
        Vec2f{x: self.x * rhs, y: self.y * rhs}
    }
}