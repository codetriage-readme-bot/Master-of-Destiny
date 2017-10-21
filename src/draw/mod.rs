use std;
use std::cell::RefCell;

use tcod::{OffscreenConsole, RootConsole};
use tcod::colors::Color;
use tcod::console;
use tcod::console::{BackgroundFlag, Console};

use life::{DrawableLiving, MissionResult};

use worldgen::{AnimalHandlerEvent, Frames, World, WorldState};
use worldgen::terrain::{TILES, Tile};

pub trait DrawChar {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize));
}

pub trait FramedDraw {
    fn draw_framed_char(&self,
                        root: &mut RootConsole,
                        pos: (usize, usize),
                        time: usize,
                        frames_hash: &Frames);
}

pub trait Describe {
    fn describe(&self) -> String;
}

fn draw_hud(root: &mut RootConsole,
            world: &WorldState,
            world_map: &World,
            wid: usize,
            hig: usize) {
    let frame_start_pos = (wid as i32 / 3) * 2 - 10;
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
    let mut hud_info: [String; 7] =
        [format!("Height: {}", world.level),
         format!("Screen Position: {}, {}",
                 world.screen.0,
                 world.screen.1),
         format!("Date: {}", world.time.calendar.describe()),
         format!("Clock: {}", world.time.clock.describe()),
         format!("Weather: {:?}", world.time.calendar.weather),
         String::new(),
         String::new()];
    let (cx, cy) = (world.cursor.0, world.cursor.1);
    if (cx >= 0 && cx < wid as i32) && (cy >= 0 && cy < hig as i32) {
        let (cx, cy) = (cx as usize, cy as usize);
        if let Some(ref wmap) = world_map.get((cx, cy)) {
            let wmapt = wmap.tiles.borrow();
            let len = wmapt.len().checked_sub(1).unwrap_or(0);
            hud_info[6] = if len < world.level as usize {
                              wmapt.get(len as usize)
                          } else {
                              wmapt.get(world.level as usize)
                          }
                          .unwrap_or(&Tile::Empty)
                          .describe();
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
                    if let Some(ref wmap) = world_map.get((mx, my)) {
                        let wmapt = wmap.tiles.borrow();
                        let len =
                            wmapt.len().checked_sub(1).unwrap_or(0);

                        match wmapt.get(world.level as usize) {
                            None => {
                                wmapt.get(len)
                                 .unwrap_or(&Tile::Empty)
                                 .draw_framed_char(root,
                                                   (x, y),
                                                   time,
                                                   &world_map.frames);
                                if TILES {
                                    let raw_c = (256 as usize)
                                        .checked_sub((world.level as
                                                          usize -
                                                          len) *
                                                         6)
                                        .unwrap_or(0);
                                    let c = std::cmp::max(raw_c,
                                                          10) as
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
                            Some(tile) => {
                                tile.draw_char(root, (x, y))
                            }
                        }
                    }
                }
            }

            let (cx, cy) = (world.cursor.0, world.cursor.1);
            if (cx >= 0 && cx < wid as i32) &&
                (cy >= 0 && cy < hig as i32)
            {
                root.set_char_background(cx as i32,
                                         cy as i32,
                                         Color::new(100, 100, 100),
                                         BackgroundFlag::Darken);
                if TILES {
                    root.set_char_foreground(cx as i32,
                                             cy as i32,
                                             Color::new(100,
                                                        100,
                                                        100));
                }
            }
            if show_hud {
                draw_hud(root, world, world_map, wid, hig);
            }
        }
        None => {}
    }
    let mut last = vec![];
    last = draw_life(root, world, last);
}

fn draw_life(root: &mut RootConsole,
             ws: &WorldState,
             last: Vec<DrawableLiving>)
    -> Vec<DrawableLiving> {
    use std::time::Duration;

    ws.life_send_tc
      .send(AnimalHandlerEvent::Draw);
    let drawables =
        match ws.life_receive_tc
                  .recv_timeout(Duration::from_millis(16)) {
            Ok(MissionResult::List(d)) => Some(d),
            _ => None,
        }
        .unwrap_or(last);

    let wid = root.width();
    let hig = root.height();
    // TODO: maybe wait for specfic MissionResult?

    {
        for l in drawables.iter() {
            let pnt = (l.current_pos.0, l.current_pos.1);
            let rel_pnt = (pnt.0 as i32 - ws.screen.0,
                           pnt.1 as i32 - ws.screen.1);
            if rel_pnt.0 < wid && rel_pnt.1 < hig &&
                rel_pnt.0 >= 0 && rel_pnt.1 >= 0 &&
                ws.level >= l.current_pos.2 as i32
            {
                l.draw_char(root,
                            (rel_pnt.0 as usize, rel_pnt.1 as usize));
            }
        }
    }
    drawables
}
