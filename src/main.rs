#![feature(box_syntax, box_patterns)]
extern crate tcod;

use std::time::{SystemTime, UNIX_EPOCH};

use tcod::{FontLayout, RootConsole};
use tcod::Console;
use tcod::input;
use tcod::input::KeyCode;

mod life;
mod draw;
mod physics;
mod worldgen;

use draw::draw_map;
use physics::liquid;
use physics::stone;
use worldgen::{World, WorldState, clamp};

fn main() {
    let screen_size = (120, 60);
    let mut root = RootConsole::initializer()
        .size(screen_size.0, screen_size.1)
        .title("The Game of Life")
        .font("cheepicus16x16_ro.png", FontLayout::AsciiInRow)
        .init();

    tcod::system::set_fps(20);
    root.set_keyboard_repeat(0, 0);

    let size = (240 as usize, 240 as usize);
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH);

    let world = World::new(size, since_the_epoch.unwrap().as_secs() as u32);

    let mut world_state = WorldState::new(world, size);
    let highest_world = world_state.highest_level as i32 - 1;
    let mut show_hud = true;
    let move_dist = 10;

    let max_screen_move = (size.0 as i32 - screen_size.0 - 1,
                           size.1 as i32 - screen_size.1 - 1);
    while !root.window_closed() {
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
                                                    move_dist as i32,
                                                    max_screen_move.1,
                                                    0);
                                    world_state.screen =
                                        (world_state.screen.0, new);
                                }
                                KeyCode::Down => {
                                    let new = clamp(world_state.screen.1 +
                                                    move_dist as i32,
                                                    max_screen_move.1,
                                                    0);
                                    world_state.screen =
                                        (world_state.screen.0, new);
                                }
                                KeyCode::Left => {
                                    let new = clamp(world_state.screen.0 -
                                                    move_dist as i32,
                                                    max_screen_move.0,
                                                    0);
                                    world_state.screen =
                                        (new, world_state.screen.1);
                                }
                                KeyCode::Right => {
                                    let new = clamp(world_state.screen.0 +
                                                    move_dist as i32,
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
        draw_map(&mut root, &world_state, show_hud);
    }
}
