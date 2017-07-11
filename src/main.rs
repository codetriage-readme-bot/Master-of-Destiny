#![feature(box_syntax, box_patterns)]
extern crate tcod;
extern crate tcod_sys;

use tcod::{FontLayout, Renderer, RootConsole};
use tcod::colors::*;
use tcod::console::{BackgroundFlag, Console};
use tcod::input;
use tcod::input::KeyCode;

mod life;
mod draw;
mod physics;
mod worldgen;
mod time;

use draw::draw_map;
use physics::liquid;
use physics::stone;
use worldgen::{World, WorldState, clamp};
use worldgen::terrain::{BASE, TILES};

const SHOW_FONT: &'static str = "assets/master16x16_ro.png";
const DEV_FONT: &'static str = "assets/terminal12x12_gs_ro.png";

const SHOW_SIZE: (i32, i32) = (100, 60);
const DEV_SIZE: (i32, i32) = (150, 75);
const MAP_SIZE: (usize, usize) = (160, 160);

const MOVE_DIST: i32 = 5;
const CYCLE_LENGTH: usize = 500;

unsafe fn load_custom_font(rows: usize) {
    let mut loc = BASE;
    for y in 16..(16 + rows) {
        tcod_sys::TCOD_console_map_ascii_codes_to_font(loc as i32,
                                                       16,
                                                       0,
                                                       y as i32);
        loc += 16;
    }
}

fn main() {
    let screen_size = SHOW_SIZE;
    let mut root = RootConsole::initializer()
        .size(screen_size.0, screen_size.1)
        .title("Skyspace")
        .font_dimensions(16, 19)
        .font(SHOW_FONT, FontLayout::AsciiInRow)
        .init();
    if TILES {
        unsafe {
            load_custom_font(2);
        }
    }

    tcod::system::set_fps(20);
    root.set_keyboard_repeat(0, 0);

    let mut world_time = time::get_world_time();
    let world = World::new(MAP_SIZE, world_time as u32);

    let mut world_state = WorldState::new(world, MAP_SIZE);
    let max_screen_move = (MAP_SIZE.0 as i32 - screen_size.0 - 1,
                           MAP_SIZE.1 as i32 - screen_size.1 - 1);
    let highest_world = world_state.highest_level as i32 - 1;

    let mut show_hud = true;
    let mut time = 0;
    while !root.window_closed() {
        time += time::get_world_time() - world_time;
        time %= CYCLE_LENGTH;
        world_time = time::get_world_time();

        world_state.time_of_day = time::calculate_time_of_day(time,
                                                              CYCLE_LENGTH);

        match input::check_for_event(input::KEY | input::MOUSE) {
            None => {}
            Some((_, event)) => {
                match event {
                    input::Event::Mouse(ref mouse) => {
                        world_state.cursor = (mouse.cx as i32,
                                              mouse.cy as i32);
                    }
                    input::Event::Key(ref key) => {
                        if key.pressed {
                            match key.code {
                                KeyCode::Tab => {
                                    show_hud = !show_hud;
                                }
                                KeyCode::Char => {
                                    match key.printable {
                                        '<' => {
                                            world_state.level =
                                                clamp(world_state.level -
                                                      1,
                                                      highest_world,
                                                      0);
                                        }
                                        '>' => {
                                            world_state.level =
                                                clamp(world_state.level +
                                                      1,
                                                      highest_world,
                                                      0);
                                        }
                                        _ => {}
                                    };
                                }
                                KeyCode::Up => {
                                    let new = clamp(world_state.screen.1 -
                                                    MOVE_DIST,
                                                    max_screen_move.1,
                                                    0);
                                    world_state.screen =
                                        (world_state.screen.0, new);
                                }
                                KeyCode::Down => {
                                    let new = clamp(world_state.screen.1 +
                                                    MOVE_DIST,
                                                    max_screen_move.1,
                                                    0);
                                    world_state.screen =
                                        (world_state.screen.0, new);
                                }
                                KeyCode::Left => {
                                    let new = clamp(world_state.screen.0 -
                                                    MOVE_DIST,
                                                    max_screen_move.0,
                                                    0);
                                    world_state.screen =
                                        (new, world_state.screen.1);
                                }
                                KeyCode::Right => {
                                    let new = clamp(world_state.screen.0 +
                                                    MOVE_DIST,
                                                    max_screen_move.0,
                                                    0);
                                    world_state.screen =
                                        (new, world_state.screen.1);
                                }
                                _ => {}
                            };
                        }
                    }
                }
            }
        }
        draw_map(&mut root, &world_state, show_hud, time);
    }

    unsafe {
        world_state.map.delete_heightmap();
    }
}
