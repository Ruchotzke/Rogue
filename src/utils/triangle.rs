use std::fmt::Debug;
use crate::utils::circle::Circle;
use crate::utils::vec2::Vec2;
use crate::utils::vec2f::Vec2f;

pub struct Triangle {
    a: Vec2f,
    b: Vec2f,
    c: Vec2f
}

impl Triangle {
    pub fn new(a: Vec2f, b: Vec2f, c: Vec2f) -> Triangle {
        Triangle { a, b, c }
    }

    /// Compute the circumcircle of this triangle
    pub fn circumcircle(&self) -> Circle {
        /* Compute two midpoints */
        let ab = (Vec2f{x: self.a.x + self.b.x, y: self.a.y + self.b.y}) * 0.5;
        let ac = (Vec2f{x: self.a.x + self.c.x, y: self.a.y + self.c.y}) * 0.5;

        /* Compute the lines through those midpoints perp to edges */
        let mut ab_vert = false;
        let mut ac_vert = false;
        let ab_slope: f64 = match self.b.x == self.a.x {
            true => {ab_vert = true; 0.0}
            false => {(self.b.y - self.a.y) / (self.b.x - self.a.x)}
        };
        let ac_slope: f64 = match self.c.x == self.a.x {
            true => {ac_vert = true; 0.0}
            false => {(self.c.y - self.a.y) / (self.c.x - self.a.y)}
        };

        let circumcenter: Vec2f = match (ab_vert, ac_vert) {
            (false, false) => {
                /* Nothing to worry about */
                let ab_slope = -1.0/ab_slope;
                let ac_slope = 1.0/ac_slope;

                let ab_b = (-ab_slope * ab.x) + ab.y;
                let ac_b = (-ac_slope * ac.x) + ac.y;

                /* Compute intersection (circumcenter) */
                let x = (ac_b - ab_b) / (ab_slope - ac_slope);
                let y = ab_b + x * ab_slope;
                Vec2f{x, y }
            },
            (false, true) => {
                /* The perp bisector of ac is flat */
                let ab_slope = -1.0/ab_slope;
                let ac_slope = 0.0;

                let ab_b = (-ab_slope * ab.x) + ab.y;
                let ac_b = (-ac_slope * ac.x) + ac.y;

                /* Compute intersection (circumcenter) */
                let x = (ac_b - ab_b) / (ab_slope - ac_slope);
                let y = ab_b + x * ab_slope;
                Vec2f{x, y }
            },
            (true, false) => {
                /* The perp bisector of ab is flat */
                let ab_slope = 0.0;
                let ac_slope = 1.0/ac_slope;

                let ab_b = (-ab_slope * ab.x) + ab.y;
                let ac_b = (-ac_slope * ac.x) + ac.y;

                /* Compute intersection (circumcenter) */
                let x = (ac_b - ab_b) / (ab_slope - ac_slope);
                let y = ab_b + x * ab_slope;
                Vec2f{x, y }
            }
            (true, true) => {
                /* This should never happen */
                panic!("Bad! Two parallel lines don't isct!")
            }
        };


        /* Compute radius from circumcenter */
        let rad = Vec2f::dist(&circumcenter, &self.a);

        /* Return the circle */
        Circle::new(circumcenter, rad)
    }
}