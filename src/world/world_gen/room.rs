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
}