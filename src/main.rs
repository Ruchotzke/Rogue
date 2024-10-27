extern crate pancurses;

mod utils;
mod player;
mod game;

use pancurses::{initscr, endwin, Input, init_color, COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_BLACK, init_pair, COLOR_WHITE, COLOR_PAIR, start_color, Window, resize_term};
use player::Player;
use game::GameInfo;

const MAP_SIZE: (u32, u32) = (25, 50);

fn main() {
    /* Window Setup */
    let window = initscr();
    resize_term(MAP_SIZE.0 as i32, MAP_SIZE.1 as i32);
    start_color();
    pancurses::noecho();
    pancurses::curs_set(0);

    /* Initialize color pairs (fg, bg) */
    init_pair(1, COLOR_GREEN, COLOR_BLACK);

    /* Initialize game information */
    let mut state = GameInfo {
        player: Player::new('@', utils::vec2::Vec2::zero())
    };

    /* Render initial screen */
    render(&window, &state);

    /* Main Game Loop */
    loop{
        /* Gather input */
        let input = window.getch();

        /* Handle input and update state */
        input_handler(input, &mut state);

        /* Render */
        render(&window, &state);
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
        _ => {}
    }
}

fn render(window: &Window, game: &GameInfo) {
    /* Clear any previous content */
    window.clear();

    /* Draw the player */
    window.attron(COLOR_PAIR(1));
    window.mvaddch(game.player.pos.y, game.player.pos.x, game.player.symbol);
    window.attroff(COLOR_PAIR(1));
}