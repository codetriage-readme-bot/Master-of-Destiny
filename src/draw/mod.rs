use tcod::{OffscreenConsole, RootConsole};
use tcod::console;
use tcod::console::{BackgroundFlag, Console};
use worldgen::{Tile, WorldState};

pub trait DrawChar {
    fn draw_char(&self, root: &mut RootConsole, pos: (usize, usize));
}

pub fn draw_map(root: &mut RootConsole,
                world: &WorldState,
                show_hud: bool) {
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
            match world.map[my][mx]
                .tiles
                .get(world.level as usize) {
                    None => (Tile::Empty).draw_char(root, (x, y)),
                    Some(tile) => tile.draw_char(root, (x, y)),
                }
        }
    }

    if show_hud {
        let frame_start_pos = (wid as i32 / 3) * 2;
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
        console::blit(window,
                      (0, 0),
                      (frame_width, frame_height),
                      root,
                      (frame_start_pos, 0),
                      1.0,
                      0.3);
    }
    root.flush();
}
