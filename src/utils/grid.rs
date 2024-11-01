use crate::utils::vec2::Vec2;

/// Perform bresenham's algorithm to generate a line between two points.
pub fn bresenham(a: &Vec2, b: &Vec2) -> Vec<Vec2> {
    let mut points = Vec::new();
    let (mut x, mut y) = (a.x, a.y);
    let dx = (b.x - a.x).abs();
    let dy = (b.y - a.y).abs();
    let sx = if a.x < b.x { 1 } else { -1 };
    let sy = if a.y < b.y { 1 } else { -1 };
    let mut err = dx - dy; //if dx > dy { dx } else { -dy } / 2;

    loop {
        points.push(Vec2::new(x, y));
        if x == b.x && y == b.y { break; }

        let e2 = err * 2;

        if e2 > -dy {
            err -= dy;
            x += sx;
        } else {
            err += dx;
            y += sy;
        }
    }
    points
}
