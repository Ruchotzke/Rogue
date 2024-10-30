use rand::Rng;
use crate::utils::vec2::Vec2;
use crate::world::map::Map;
use crate::world::world_gen::room::Room;
use super::room;

const ROOM_SIZE_MIN: u16 = 4;
const ROOM_SIZE_MAX: u16 = 20;

/// Generate a map filled with rooms.
pub fn generate_map(map_size: Vec2, min_rooms: u16, max_rooms: u16) -> Map {
    /* First generate an empty map */
    let mut map = Map::new(map_size.x as u32, map_size.y as u32);

    let room_size_min = Vec2::new(ROOM_SIZE_MIN as i32, ROOM_SIZE_MIN as i32);
    let room_size_max = Vec2::new(ROOM_SIZE_MAX as i32, ROOM_SIZE_MAX as i32);

    /* Now generate some rooms */
    /* If we can't get the proper number of rooms within 100 iterations, something is horrifically wrong */
    for _iter in 0..100 {
        let mut rooms:Vec<Room> = Vec::new();
        let mut regen_attempts = 10000;

        /* Attempt to generate max_rooms */
        for _i in 0..max_rooms {
            /* Inner loop used for regeneration */
            while regen_attempts > 0 {
                /* Generate a room */
                let r = Room::rand_new(
                    &mut rand::thread_rng(),
                    &map_size,
                    &room_size_min,
                    &room_size_max,
                ).unwrap();

                /* See if it intersects with any other rooms. */
                /* If it does, we need to regenerate it */
                let mut conflict = false;
                for other in &rooms {
                    if(other.intersects(&r)){
                        conflict = true;
                        break;
                    }
                }

                /* If there was a conflict, regenerate if possible */
                if conflict {
                    regen_attempts -= 1;
                    continue;
                }

                /* If there wasn't a conflict, add and move on */
                rooms.push(r);
                break;
            }
        }

        /* Check the amount actually generated */
        if rooms.len() >= min_rooms as usize {
            /* If we're in spec, we're done */

            /* Add all rooms into the map */
            for room in &rooms {
                map.add_room(room);
            }

            /* Connect all rooms to another room */
            for i in 1..rooms.len() {
                /* Connect the two */
                map.add_hall(&rooms[i], &rooms[i-1]);
            }

            break;
        }
        else{
            /* If we're not in spec, loop */
            continue;
        }
    }

    return map;
}
