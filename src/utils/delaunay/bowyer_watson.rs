use crate::utils::delaunay::utils::get_enclosing_triangle;
use crate::utils::vec2f::Vec2f;

/// Compute the delaunay triangulation of a set of points.
pub fn bowyer_watson(points: &Vec<Vec2f>) -> Vec<(Vec2f, Vec2f)> {
    /* Construct super-triangle */
    let mut sup = get_enclosing_triangle(points);


}