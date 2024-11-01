use crate::utils::triangle::Triangle;
use crate::utils::vec2f::Vec2f;

/// Get the extremes of a given set.
pub fn get_extremes(points: &Vec<Vec2f>) -> (Vec2f, Vec2f, f64, f64) {
    if points.len() < 3 {panic!("Too few!");}
    let mut min = Vec2f::new(points[0].x, points[0].y);
    let mut max = Vec2f::new(points[0].x, points[0].y);

    /* Compute */
    for i in 1..points.len() {
        let p = &points[i];
        if p.x < min.x {min.x = p.x;}
        if p.y < min.y {min.y = p.y;}
        if p.x > max.x {max.x = p.x;}
        if p.y > max.y {max.y = p.y;}
    }

    /* Return the info */
    let width = max.x - min.x;
    let height = max.y - min.y;
    (min, max, width, height)
}

pub fn get_enclosing_triangle(points: &Vec<Vec2f>) -> Triangle{
    /* First get extremes */
    let (min, max, width, height) = get_extremes(points);

    Triangle {
        a: Vec2f::new(min.x - width * 0.1, min.y - height),
        b: Vec2f::new(min.x - width * 0.1, min.y + height * 2.0),
        c: Vec2f::new(min.x + width * 1.7, min.y + height * 0.5)
    }
}