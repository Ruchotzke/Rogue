extern crate pancurses;

mod utils;
mod player;
mod game;
mod world;

use pancurses::{initscr, endwin, Input, init_color, COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_BLACK, init_pair, COLOR_WHITE, COLOR_PAIR, start_color, Window, resize_term};
use player::Player;
use game::GameInfo;
use crate::utils::vec2::Vec2;
use crate::world::cell::{Cell, CellAccess};
use crate::world::map::Map;
use crate::world::world_gen::generate_level::generate_map;

const MAP_SIZE: (u32, u32) = (90, 30);

fn main() {
    /* Window Setup */
    let window = initscr();
    resize_term(MAP_SIZE.1 as i32, MAP_SIZE.0 as i32);
    start_color();
    pancurses::noecho();
    pancurses::curs_set(0);

    /* Initialize color pairs (fg, bg) */
    init_pair(1, COLOR_GREEN, COLOR_BLACK);

    /* Initialize game information */
    let mut state = GameInfo {
        player: Player::new('@', utils::vec2::Vec2::zero()),
        map: generate_map(Vec2::new(MAP_SIZE.0 as i32, MAP_SIZE.1 as i32), 4, 15)
    };

    /* Render initial screen */
    render(&window, &mut state);

    /* Main Game Loop */
    loop{
        /* Gather input */
        let input = window.getch();

        /* Handle input and update state */
        input_handler(input, &mut state);

        /* Render */
        render(&window, &mut state);
        window.refresh();

    }

    println!("Exiting gracefully...");
    endwin();
}

fn input_handler(input: Option<Input>, game: &mut GameInfo) {
    match input {
        Some(Input::Character('w')) =>{
            game.player.pos.y -= 1;
        },
        Some(Input::Character('s')) =>{
            game.player.pos.y += 1;
        },
        Some(Input::Character('a')) =>{
            game.player.pos.x -= 1;
        }
        Some(Input::Character('d')) =>{
            game.player.pos.x += 1;
        }
        Some(Input::Character(' ')) => {
            game.map = generate_map(Vec2::new(MAP_SIZE.0 as i32, MAP_SIZE.1 as i32), 4, 15)
        }
        _ => {}
    }
}

fn render(window: &Window, game: &mut GameInfo) {
    /* Clear any previous content */
    window.clear();

    /* Draw the map */
    for x in 0..game.map.width {
        for y in 0..game.map.height {
            match game.map.get_cell(x, y).unwrap().access {
                CellAccess::OPEN => {
                    window.mvaddch(y as i32, x as i32, '#');
                }
                CellAccess::CLOSED => {
                    window.mvaddch(y as i32, x as i32, ' ');
                }
            }
        }
    }

    /* Draw the player */
    window.attron(COLOR_PAIR(1));
    window.mvaddch(game.player.pos.y, game.player.pos.x, game.player.symbol);
    window.attroff(COLOR_PAIR(1));
}