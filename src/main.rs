extern crate tcod;

use std::time::{SystemTime, UNIX_EPOCH};

use tcod::{FontLayout, RootConsole};
use tcod::Console;
use tcod::input;

mod life;
mod draw;
mod physics;
mod worldgen;

use draw::draw_map;
use physics::liquid;
use physics::stone;
use worldgen::{World, WorldState};

fn main() {
    let mut root = RootConsole::initializer()
        .size(80, 30)
        .title("God Gandorf")
        .font("terminal12x12_gs_ro.png", FontLayout::AsciiInRow)
        .init();

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH);
    let mut world_state =
        WorldState::new(World::new((120, 120),
                                   since_the_epoch.unwrap().as_secs() as
                                   u32));
    while !root.window_closed() {
        match input::check_for_event(input::MOUSE | input::KEY) {
            None => {}
            Some((_, event)) => {
                match event {
                    input::Event::Key(ref key_state) => {
                        println!("{:?}", key_state);
                    }
                    input::Event::Mouse(ref mouse_state) => {
                        let x = mouse_state.cx as i32;
                        let y = mouse_state.cy as i32;

                        println!("{:?}", mouse_state);
                    }
                }
            }
        }
        root.clear();
        draw_map(root, world_state);
        root.flush();
    }
}
