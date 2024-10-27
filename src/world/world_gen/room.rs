use rand::Rng;
use crate::utils::vec2::Vec2;

/// A room on the map.
/// Rectangular, has a size and origin.
pub struct Room {
    pub origin: Vec2,
    pub size: Vec2,
}

impl Room {

    /// Generate a new random room
    /// or_max: world size
    /// s_min: minimum x/y sizes
    /// s_max: maximum x/y sizes
    pub fn rand_new(rng: &mut rand::rngs::ThreadRng, or_max: Vec2, s_min: Vec2, s_max: Vec2) -> Room {
        /* Generate an origin */
        let origin = Vec2::new(rng.gen_range(0..or_max.x), rng.gen_range(0..or_max.y));

        /* Generate a room size */
        let size = Vec2::new(rng.gen_range(s_min.x..s_max.x), rng.gen_range(s_min.y..s_max.y));

        /* Return the generated room */
        Room { origin, size }
    }

    /// Get the bounds (min and max) for both the x and y directions.
    pub fn get_bounds(&self) -> (Vec2, Vec2) {
       (Vec2::new(self.origin.x, self.origin.y), Vec2::new(self.origin.x + self.size.x, self.origin.y + self.size.y))
    }

    /// Compute whether this room intersects with another.
    pub fn intersects(&self, other: &Room) -> bool {
        /* Get the bounds of the other room */
        let bnd = other.get_bounds();

        /* Iterate over every cell */
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                /* Determine isct */
                if x >= bnd.0.x && x < bnd.1.x && y >= bnd.0.y && y < bnd.1.y{
                    return false;
                }
            }
        }

       true
    }
}