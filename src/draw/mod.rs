use tcod::{OffscreenConsole, RootConsole};
use tcod::colors::Color;
use tcod::console;
use tcod::console::{BackgroundFlag, Console};
use worldgen::WorldState;
use worldgen::terrain::Tile;

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
            let tiles = &world.map[my][mx].tiles;
            let len = tiles.len().checked_sub(1).unwrap_or(0);

            match tiles.get(world.level as usize) {
                None => {
                    tiles.get(len)
                        .unwrap_or(&Tile::Empty)
                        .draw_char(root, (x, y));
                }
                Some(tile) => tile.draw_char(root, (x, y)),
            }
        }
    }

    if show_hud {
        let frame_start_pos = (wid as i32 / 3) * 2 + 5;
        let frame_width = wid as i32 - frame_start_pos;
        let frame_height = hig as i32;
        let mut window = &OffscreenConsole::new(frame_width, frame_height);
        window.print_frame(0,
                           0,
                           frame_width,
                           frame_height,
                           true,
                           BackgroundFlag::Default,
                           Some("Tools"));
        window.print(1, 1, format!("Height: {}", world.level));
        window.print(1,
                     2,
                     format!("Screen Position: {}, {}",
                             world.screen.0,
                             world.screen.1));
        if (world.cursor.0 >= 0 &&
            world.cursor.0 < world.map[0].len() as i32) &&
            (world.cursor.1 >= 0 &&
             world.cursor.1 < world.map.len() as i32)
        {
            let (cx, cy) = (world.cursor.0 as usize,
                            world.cursor.1 as usize);
            let tiles = &world.map[cy][cx].tiles;
            let len = tiles.len().checked_sub(1).unwrap_or(0) as i32;
            window.print(1,
                         3,
                         if len < world.level {
                             tiles.get(len as usize)
                                 .unwrap_or(&Tile::Empty)
                                 .describe()
                         } else {
                             tiles.get(world.level as usize)
                                 .unwrap_or(&Tile::Empty)
                                 .describe()
                         });
            root.set_char_background(cx as i32,
                                     cy as i32,
                                     Color::new(255, 255, 0),
                                     BackgroundFlag::Lighten);
        }
        window.print(1,
                     4,
                     format!("Time of Day: {:?}", world.time_of_day));
        window.print(1,
                     5,
                     format!("Percentage of Night/Day Cycle: {}",
                             time as f32 / 500 as f32));
        console::blit(window,
                      (0, 0),
                      (frame_width, frame_height),
                      root,
                      (frame_start_pos, 0),
                      1.0,
                      0.6);
    }
    root.flush();
}
