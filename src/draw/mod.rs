use std;
use tcod::{OffscreenConsole, RootConsole};
use tcod::colors::Color;
use tcod::console;
use tcod::console::{BackgroundFlag, Console};

use time;
use worldgen::{CYCLE_LENGTH, WorldState};
use worldgen::terrain::{TILES, Tile};

pub trait DrawChar {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize));
}

pub trait Describe {
    fn describe(&self) -> String;
}

pub fn draw_map(root: &mut RootConsole,
                world: &WorldState,
                show_hud: bool,
                time: usize) {
    match world.map {
        Some(ref wmap) => {
            let world_map = wmap;
            root.clear();
            let wid = root.width() as usize;
            let hig = root.height() as usize;

            let screen_start_y = world.screen.1 as usize;
            let screen_end_y = screen_start_y + hig;

            let screen_start_x = world.screen.0 as usize;
            let screen_end_x = screen_start_x + wid;

            for (my, y) in (screen_start_y..screen_end_y)
                .zip(0..hig)
            {
                for (mx, x) in (screen_start_x..screen_end_x)
                    .zip(0..wid)
                {
                    let wmap = &world_map[my][mx];
                    let len = wmap.tiles
                                  .len()
                                  .checked_sub(1)
                                  .unwrap_or(0);

                    match wmap.tiles.get(world.level as usize) {
                        None => {
                            wmap.tiles
                                .get(len)
                                .unwrap_or(&Tile::Empty)
                                .draw_char(root, (x, y));
                            if TILES {
                                let raw_c = (256 as usize)
                                    .checked_sub((world.level as
                                                      usize -
                                                      len) *
                                                     8)
                                    .unwrap_or(0);
                                let c = std::cmp::max(raw_c, 10) as
                                    u8;
                                root.set_char_foreground(x as i32,
                                                         y as i32,
                                                         Color::new(c,
                                                                    c,
                                                                    c));
                            } else {
                                root.set_char_background(x as i32,
                                             y as i32,
                                             Color::new(100, 100, 100),
                                             BackgroundFlag::Darken);
                            }
                        }
                        Some(tile) => tile.draw_char(root, (x, y)),
                    }
                }
            }

            if show_hud {
                let frame_start_pos = (wid as i32 / 3) * 2 + 5;
                let frame_width = wid as i32 - frame_start_pos;
                let frame_height = hig as i32;
                let mut window = &OffscreenConsole::new(frame_width,
                                                        frame_height);
                window.print_frame(0,
                                   0,
                                   frame_width,
                                   frame_height,
                                   true,
                                   BackgroundFlag::Set,
                                   Some("Tools"));
                let mut hud_info: [String; 8] =
                    [format!("Height: {}", world.level),
                     format!("Screen Position: {}, {}",
                             world.screen.0,
                             world.screen.1),
                     format!("ToD: {}",
                             world.time_of_day.describe()),
                     format!("Date: {}", world.calendar.describe()),
                     format!("Clock: {}", world.clock.describe()),
                     format!("Weather: {:?}",
                             world.calendar.weather),
                     String::new(),
                     String::new()];
                if (world.cursor.0 >= 0 &&
                        world.cursor.0 < world_map[0].len() as i32) &&
                    (world.cursor.1 >= 0 &&
                         world.cursor.1 < world_map.len() as i32)
                {
                    let (cx, cy) = (world.cursor.0 as usize,
                                    world.cursor.1 as usize);
                    let wmap = &world_map[cy][cx];
                    let len = wmap.tiles
                                  .len()
                                  .checked_sub(1)
                                  .unwrap_or(0);
                    hud_info[6] = if len < world.level as usize {
                                      wmap.tiles.get(len as usize)
                                  } else {
                                      wmap.tiles
                                          .get(world.level as usize)
                                  }
                                  .unwrap_or(&Tile::Empty)
                                  .describe();
                    if len != world.level as usize {
                        hud_info[7] = format!("Distance from Level: {}",
                                              world.level as i32 -
                                                  len as i32);
                    }
                    root.set_char_background(cx as i32,
                                             cy as i32,
                                             Color::new(100,
                                                        100,
                                                        100),
                                             BackgroundFlag::Darken);
                    if TILES {
                        root.set_char_foreground(cx as i32,
                                                 cy as i32,
                                                 Color::new(100,
                                                            100,
                                                            100));
                    }
                }
                for (i, line) in hud_info.iter().enumerate() {
                    window.print(1, i as i32 + 1, line);
                }
                console::blit(window,
                              (0, 0),
                              (frame_width, frame_height),
                              root,
                              (frame_start_pos, 0),
                              1.0,
                              0.6);
            }
        }
        None => {}
    }
}
