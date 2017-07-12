#![feature(box_syntax, box_patterns)]
extern crate tcod;
extern crate tcod_sys;

use tcod::{FontLayout, Renderer, RootConsole};
use tcod::colors::*;
use tcod::console::{BackgroundFlag, Console, TextAlignment};
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

const SHOW_SIZE: (i32, i32) = (120, 64);
const DEV_SIZE: (i32, i32) = (150, 75);
const MAP_SIZE: (usize, usize) = (160, 160);

const MOVE_DIST: i32 = 5;
const CYCLE_LENGTH: usize = 500;

const GAME: bool = true;
const TITLE_CARD: [&'static str; 6] = ["   _________ __",
                                       "  /   _____/|  | __ ___.__.  ____________  _____     ____    ____",
                                       "  \\_____  \\ |  |/ /<   |  | /  ___/\\____ \\ \\__  \\  _/ ___\\ _/ __ \\",
                                       "  /        \\|    <  \\___  | \\___ \\ |  |_> > / __ \\_\\  \\___ \\  ___/",
                                       " /_______  /|__|_ \\ / ____|/____  >|   __/ (____  / \\___  > \\___  >",
                                       " \\/      \\/      \\/             \\/ |__|         \\/      \\/      \\/"];

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

enum GameScreen {
    Menu,
    Game,
    Loading,
    Paused,
}

struct Calc {
    pub max_screen_move: (i32, i32),
    pub highest_world: i32,
    pub screen_size: (i32, i32),
}

struct Menu {
    bboxes: Vec<((i32, i32), (i32, i32))>,
    start_pos: (i32, i32),
    button_size: (i32, i32),
    pub items: Vec<String>,
}

impl Menu {
    pub fn calculate_bboxes(pos: (i32, i32),
                            number: i32,
                            size: (i32, i32))
        -> Vec<((i32, i32), (i32, i32))> {
        (0..number)
            .map(|n| {
                let y = pos.1 + n * size.1;
                ((pos.0, y), (pos.0 + size.0, y))
            })
            .collect()
    }

    pub fn bbox_colliding(&self, loc: (i32, i32)) -> Option<String> {
        for (i, &(bbs, bbe)) in self.bboxes.iter().enumerate() {
            if (loc.0 >= bbs.0 && loc.1 <= bbs.1) &&
                (loc.0 <= bbe.0 && loc.1 >= bbe.1)
            {
                return Some(self.items[i].clone());
            }
        }
        None
    }

    pub fn draw(&self,
                mut root: &mut RootConsole,
                world_state: &WorldState) {
        root.set_default_foreground(Color::new(0, 0, 0));
        for (i, &(bbs, _)) in self.bboxes.iter().enumerate() {
            let item = self.items[i].clone();
            if self.bbox_colliding(world_state.cursor)
                   .unwrap_or("Nothing".to_string()) ==
                *item
            {
                root.set_default_background(Color::new(255, 255, 255));
            } else {
                root.set_default_background(Color::new(100, 100, 100));
            }

            root.print_ex(bbs.0,
                          bbs.1,
                          BackgroundFlag::Set,
                          TextAlignment::Center,
                          item);
        }
        root.set_default_foreground(Color::new(255, 255, 255));
        root.set_default_background(Color::new(0, 0, 0));
    }
}

struct Game {
    show_hud: bool,
    constants: Calc,
    world_time: usize,
    menu: Menu,
    pub time: usize,
    pub screen: GameScreen,
    pub world_state: WorldState,
}

impl Game {
    pub fn new(screen_size: (i32, i32)) -> Game {
        Game {
            show_hud: true,
            constants: Calc {
                max_screen_move: (-1, -1),
                highest_world: 0,
                screen_size: screen_size,
            },
            world_time: 0,
            time: 0,
            screen: GameScreen::Menu,
            menu: Menu {
                start_pos: (screen_size.0 / 2, 30),
                items: vec!["   Start   ".to_string(),
                            "    Exit   ".to_string()],
                button_size: (7, 1),
                bboxes: Menu::calculate_bboxes((screen_size.0 / 2, 30),
                                               2,
                                               (7, 1)),
            },
            world_state: WorldState::new(MAP_SIZE),
        }
    }

    pub fn init_game(&mut self) {
        self.world_time = time::get_world_time();

        let world = World::new(MAP_SIZE, self.world_time as u32);
        self.world_state.add_map(world);

        self.constants.max_screen_move =
            (MAP_SIZE.0 as i32 - self.constants.screen_size.0 - 1,
             MAP_SIZE.1 as i32 - self.constants.screen_size.1 - 1);
        self.constants.highest_world = self.world_state.highest_level as
            i32 - 1;
        self.screen = GameScreen::Game;
    }

    pub fn draw(&mut self, mut root: &mut RootConsole) {
        match self.screen {
            GameScreen::Loading => {
                root.print_ex(self.constants.screen_size.0 / 2,
                              2,
                              BackgroundFlag::Set,
                              TextAlignment::Center,
                              "Loading...");
                for y in 0..20 {
                    for x in 1..17 {
                        root.put_char(x,
                                      y + 4,
                                      std::char::from_u32((x * y) as u32)
                                          .unwrap(),
                                      BackgroundFlag::None);
                    }
                }
            }
            GameScreen::Menu => {
                root.clear();
                for (i, line) in TITLE_CARD.iter().enumerate() {
                    root.print_ex(self.constants.screen_size.0 / 2 - 32,
                                  i as i32 + 2,
                                  BackgroundFlag::None,
                                  TextAlignment::Left,
                                  line.to_string());
                }

                self.menu.draw(root, &self.world_state);
            }
            GameScreen::Game => {
                self.time += time::get_world_time() - self.world_time;
                self.time %= CYCLE_LENGTH;
                self.world_time = time::get_world_time();

                self.world_state.time_of_day =
                    time::calculate_time_of_day(self.time, CYCLE_LENGTH);
                self.world_state.update();
                draw_map(&mut root,
                         &self.world_state,
                         self.show_hud,
                         self.time);
            }
            GameScreen::Paused => {
                root.print_ex(self.constants.screen_size.0 / 2,
                              2,
                              BackgroundFlag::Set,
                              TextAlignment::Center,
                              "Paused");
            }
        }
        match input::check_for_event(input::KEY | input::MOUSE) {
            None => {}
            Some((_, event)) => {
                match event {
                    input::Event::Mouse(ref mouse) => {
                        self.world_state.cursor = (mouse.cx as i32,
                                                   mouse.cy as i32);
                        self.handle_mouse(mouse);
                    }
                    input::Event::Key(ref key) => {
                        if key.pressed {
                            self.handle_key(key);
                        }
                    }
                }
            }
        }
    }

    fn move_delta(&mut self, xdelta: i32, ydelta: i32) {
        self.world_state.screen =
            (clamp(self.world_state.screen.0 + xdelta,
                   self.constants.max_screen_move.0,
                   0),
             clamp(self.world_state.screen.1 + ydelta,
                   self.constants.max_screen_move.1,
                   0))
    }
    fn handle_mouse(&mut self, mouse: &input::Mouse) {
        match self.screen {
            GameScreen::Menu => {
                if mouse.lbutton_pressed {
                    let bitem =
                        self.menu
                            .bbox_colliding(self.world_state.cursor);
                    match bitem {
                        Some(item) => {
                            match item.as_ref() {
                                "   Start   " => self.init_game(),
                                "    Exit   " => std::process::exit(0),
                                _ => {}
                            }
                        }
                        None => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, key: &input::Key) {
        match self.screen {
            GameScreen::Loading => {}
            GameScreen::Menu => {
                match key.code {
                    KeyCode::Char => self.init_game(),
                    KeyCode::Spacebar => self.init_game(),
                    _ => {}
                }
            }
            GameScreen::Paused => {
                match key.code {
                    KeyCode::Spacebar => self.screen = GameScreen::Game,
                    KeyCode::Escape => self.screen = GameScreen::Menu,
                    _ => {}
                }
            }
            GameScreen::Game => {
                match key.code {
                    KeyCode::Spacebar => self.screen = GameScreen::Paused,
                    KeyCode::Tab => {
                        self.show_hud = !self.show_hud;
                    }
                    KeyCode::Char => {
                        match key.printable {
                            '<' => {
                                self.world_state.level =
                                    clamp(self.world_state.level - 1,
                                          self.constants.highest_world,
                                          0);
                            }
                            '>' => {
                                self.world_state.level =
                                    clamp(self.world_state.level + 1,
                                          self.constants.highest_world,
                                          0);
                            }
                            _ => {}
                        };
                    }
                    KeyCode::Up => self.move_delta(0, -MOVE_DIST),
                    KeyCode::Down => self.move_delta(0, MOVE_DIST),
                    KeyCode::Left => self.move_delta(-MOVE_DIST, 0),
                    KeyCode::Right => self.move_delta(MOVE_DIST, 0),
                    _ => {}
                };
            }
        }
    }
}

fn main() {
    let screen_size = SHOW_SIZE;
    let mut root = RootConsole::initializer()
        .size(screen_size.0, screen_size.1)
        .title("Skyspace")
        .renderer(Renderer::SDL)
        .font_dimensions(16, 19)
        .font(SHOW_FONT, FontLayout::AsciiInRow)
        .init();

    if TILES {
        unsafe {
            load_custom_font(2);
        }
    }

    let mut game = Game::new(screen_size);
    for line in TITLE_CARD.iter() {
        println!("{}", line);
    }

    if GAME {
        tcod::system::set_fps(20);
        root.set_keyboard_repeat(0, 0);

        while !root.window_closed() {
            game.draw(&mut root);
            root.flush();
        }

        unsafe {
            match game.world_state.map {
                Some(x) => x.delete_heightmap(),
                None => {}
            }
        }
    }
}
