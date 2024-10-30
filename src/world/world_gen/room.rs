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
    pub fn rand_new(rng: &mut rand::rngs::ThreadRng, world_size: &Vec2, s_min: &Vec2, s_max: &Vec2) -> Result<Room, String> {
        /* We need to make sure this is possible; adjust the origin max to be s_min away */
        let max_origin= Vec2::new(world_size.x-s_min.x, world_size.y-s_min.y);
        if max_origin.x < 0 || max_origin.y < 0 {
            return Err(String::from("Cannot construct room with these arguments."));
        }

        /* Generate an origin */
        let origin = Vec2::new(rng.gen_range(0..=max_origin.x), rng.gen_range(0..=max_origin.y));

        /* Generate a room size */
        let mut size = Vec2::new(rng.gen_range(s_min.x..s_max.x), rng.gen_range(s_min.y..s_max.y));

        /* If size is pushing the corner beyond limits, clamp it */
        let (xdiff, ydiff) = (world_size.x - (origin.x + size.x), world_size.y - (origin.y + size.y));
        if xdiff < 0 {
            size.x += xdiff;
        }
        if ydiff < 0 {
            size.y += ydiff;
        }

        /* Return the generated room */
        Ok(Room { origin, size })
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
        for x in self.origin.x..(self.origin.x + self.size.x) {
            for y in self.origin.y..(self.origin.y + self.size.y) {
                /* Determine isct */
                if x >= bnd.0.x && x < bnd.1.x && y >= bnd.0.y && y < bnd.1.y{
                    return true;
                }
            }
        }

       false
    }

    /// Shrink this room by one unit in all directions if possible
    pub fn shrink(&mut self){
        if self.size.x > 1 {
            self.size.x -= 2;
            self.origin.x += 1;
        }

        if self.size.y > 1 {
            self.size.y -= 2;
            self.origin.y += 1;
        }
    }

    /// Return a random point on the interior of this room.
    pub fn get_rand_point(&self, rng: &mut rand::rngs::ThreadRng) -> Vec2 {
        Vec2::new(self.origin.x + rng.gen_range(0..self.size.x), self.origin.y + rng.gen_range(0..self.size.y))
    }
}

#[cfg(test)]
mod tests{
    use crate::utils::vec2::Vec2;
    use crate::world::world_gen::room::Room;

    #[test]
    fn test_bounds(){
        let r: Room = Room{
            size: Vec2::new(10, 11),
            origin: Vec2::new(0, 0),
        };

        assert_eq!(r.get_bounds().0.x, 0);
        assert_eq!(r.get_bounds().1.x, 10);
        assert_eq!(r.get_bounds().0.y, 0);
        assert_eq!(r.get_bounds().1.y, 11);

        let r: Room = Room{
            origin: Vec2::new(22, 19),
            size: Vec2::new(3, 7)
        };

        assert_eq!(r.get_bounds().0.x, 22);
        assert_eq!(r.get_bounds().0.y, 19);
        assert_eq!(r.get_bounds().1.x, 25);
        assert_eq!(r.get_bounds().1.y, 26);
    }

    #[test]
    fn test_shortening(){
        /* Test 1: does an error get thrown? */
        let world_size = Vec2::new(1,1);
        let min_size = Vec2::new(3,3);
        let max_size = Vec2::new(3,3);
        let mut generated = false;
        let r: Room = match Room::rand_new(&mut rand::thread_rng(), &world_size, &min_size, &max_size){
            Ok(r) => {
                generated = true;
                r
            },
            Err(e) => {
                Room {
                    origin: Vec2::zero(),
                    size: Vec2::zero()
                }
            }
        };

        assert!(!generated);

        /* Test 2, only one room possible with shortening */
        let world_size = Vec2::new(1,1);
        let min_size = Vec2::new(1,1);
        let max_size = Vec2::new(3,3);

        let r = match Room::rand_new(&mut rand::thread_rng(), &world_size, &min_size, &max_size){
            Ok(r) => r,
            Err(e) => panic!("{}", e)
        };
        assert_eq!(r.size.x, 1);
        assert_eq!(r.size.y, 1);
        assert_eq!(r.origin.x, 0);
        assert_eq!(r.origin.y, 0);
    }
    #[test]
    fn test_construction(){
        let world_size: Vec2 = Vec2::new(30,30);
        let room_min: Vec2 = Vec2::new(3, 3);
        let room_max: Vec2 = Vec2::new(22, 22);

        for _i in 0..10000 {
            /* Generate a new room */
            let room: Room = match Room::rand_new(&mut rand::thread_rng(), &world_size, &room_min, &room_max){
                Ok(r) => r,
                Err(e) => panic!("{}", e)
            };

            /* Test! */
            /* The room size is within limits */
            assert!(room.size.x >= room_min.x && room.size.x < room_max.x);
            assert!(room.size.y >= room_min.y && room.size.y < room_max.y);

            /* Make sure the bounds are within limits */
            let bounds = room.get_bounds();
            assert!(bounds.0.x >= 0 && bounds.0.y >= 0);
            assert!(bounds.1.x <= world_size.x && bounds.1.y <= world_size.y);
        }
    }

    #[test]
    fn test_intersection(){
        let r1: Room = Room{
            origin: Vec2::new(3, 3),
            size: Vec2::new(2, 2),
        };

        let r2: Room = Room {
            origin: Vec2::new(0,0),
            size: Vec2::new(2,2),
        };

        let r3: Room = Room {
            origin: Vec2::new(0, 4),
            size: Vec2::new(10, 1)
        };

        assert!(!r1.intersects(&r2));
        assert!(!r2.intersects(&r1));

        assert!(!r2.intersects(&r3));
        assert!(!r3.intersects(&r2));

        assert!(r3.intersects(&r1));
        assert!(r1.intersects(&r3));
    }
}