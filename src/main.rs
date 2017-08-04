#![feature(box_syntax, box_patterns, vec_remove_item)]
extern crate tcod;
extern crate tcod_sys;

use tcod::{FontLayout, FontType, Renderer, RootConsole};
use tcod::console::{BackgroundFlag, Console, TextAlignment};
use tcod::input;
use tcod::input::KeyCode;

mod life;
mod draw;
mod physics;
mod ui;
mod worldgen;
mod time;

use ui::{Button, DrawUI, Layout, MouseUI};
        
use draw::draw_map;

use worldgen::{World, WorldState, clamp};
use worldgen::terrain::{BASE, TILES};

const SHOW_FONT: &'static str = "assets/master16x16_ro.png";

const SHOW_SIZE: (i32, i32) = (100, 60);
const MAP_SIZE: (usize, usize) = (160, 160);

const MOVE_DIST: i32 = 5;

const DEBUG: bool = false;
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

struct Game {
    show_hud: bool,
    constants: Calc,
    last_time: usize,
    menu: Layout,
    pause_menu: Layout,
    show_tools: Button,
    pub time: usize,
    pub screen: GameScreen,
    pub world_state: WorldState,
    seed: u32,
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
            last_time: 0,
            time: 0,
            screen: GameScreen::Menu,
            show_tools: Button::new("Menu \u{1A}",
                                    (screen_size.0 - 7,
                                     screen_size.1 - 2),
                                    (6, 1)),
            menu: Layout::new(vec!["New Game", "Use Seed", "Exit"],
                              (screen_size.0 / 2, 30),
                              (8, 0),
                              8),
            pause_menu: Layout::new(vec!["Main Menu",
                                         "Back",
                                         "Fullscreen",
                                         "Exit"],
                                    (screen_size.0 / 2, 4),
                                    (10, 0),
                                    10),
            world_state: WorldState::new(MAP_SIZE),
            seed: 0,
        }
    }

    pub fn init_game(&mut self, seed: Option<u32>) {
        let lpos = self.menu.buttons[self.menu.buttons.len() - 1]
            .bbox
            .0;
        self.menu.buttons.insert(0,
                                 Button::new("Resume",
                                             (lpos.0 + 8 % 8,
                                              lpos.1 + 1),
                                             (8, 0)));
        self.last_time = time::get_world_time();
        self.seed = seed.unwrap_or(self.last_time as u32);
        let world = World::new(MAP_SIZE, self.seed);
        self.world_state.add_map(world);

        self.constants.max_screen_move =
            (MAP_SIZE.0 as i32 - self.constants.screen_size.0 - 1,
             MAP_SIZE.1 as i32 - self.constants.screen_size.1 - 1);
        self.constants.highest_world =
            self.world_state.highest_level as i32 - 1;
        self.screen = GameScreen::Game;
    }

    pub fn draw(&mut self, root: &mut RootConsole) {
        match self.screen {
            GameScreen::Loading => {
                root.clear();
                root.print_ex(self.constants.screen_size.0 / 2,
                              2,
                              BackgroundFlag::Set,
                              TextAlignment::Center,
                              "Loading...");
                for c in 0..(16 * 19) {
                    root.put_char(c % self.constants.screen_size.0,
                                  c / self.constants.screen_size.0 +
                                      3,
                                  std::char::from_u32(c as u32)
                                      .unwrap(),
                                  BackgroundFlag::Set);
                }
            }
            GameScreen::Menu => {
                root.clear();
                for (i, line) in TITLE_CARD.iter().enumerate() {
                    root.print_ex(self.constants.screen_size.0 / 2 -
                                      32,
                                  i as i32 + 2,
                                  BackgroundFlag::None,
                                  TextAlignment::Left,
                                  line.to_string());
                }

                self.menu
                    .draw(root, self.world_state.cursor);
            }
            GameScreen::Game => {
                let current_time = time::get_world_time();
                self.world_state.update(current_time,
                                        current_time -
                                            self.last_time);
                self.last_time = current_time;
                draw_map(root,
                         &self.world_state,
                         self.show_hud,
                         self.last_time);
                self.show_tools
                    .draw(root, self.world_state.cursor);
            }
            GameScreen::Paused => {
                root.clear();
                root.print_ex(self.constants.screen_size.0 / 2,
                              2,
                              BackgroundFlag::Set,
                              TextAlignment::Center,
                              "Paused");
                self.pause_menu
                    .draw(root, self.world_state.cursor);
            }
        }
        match input::check_for_event(input::KEY | input::MOUSE) {
            None => {}
            Some((_, event)) => {
                match event {
                    input::Event::Mouse(ref mouse) => {
                        self.world_state.cursor = (mouse.cx as i32,
                                                   mouse.cy as i32);
                        self.handle_mouse(root, mouse);
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
    fn handle_mouse(&mut self,
                    root: &mut RootConsole,
                    mouse: &input::Mouse) {
        match self.screen {
            GameScreen::Menu => {
                if mouse.lbutton_pressed {
                    let bitem =
                        self.menu
                            .bbox_colliding(self.world_state.cursor);
                    match bitem {
                        Some(item) => {
                            match item.trim().as_ref() {
                                "new_game" => self.init_game(None),
                                "use_seed" => self.init_game(None),
                                "resume" => {
                                    self.screen = GameScreen::Game
                                }
                                "exit" => std::process::exit(0),
                                _ => {}
                            }
                        }
                        None => {}
                    }
                }
            }
            GameScreen::Game => {
                if mouse.lbutton_pressed {
                    if self.show_tools
                           .bbox_colliding(self.world_state.cursor)
                           .is_some()
                    {
                        if self.show_hud {
                            self.show_tools.text = "Menu \u{1B}"
                                .to_string();
                            self.show_hud = false;
                        } else {
                            self.show_tools.text = "Menu \u{1a}"
                                .to_string();
                            self.show_hud = true;
                        }
                    }
                }
            }
            GameScreen::Paused => {
                if mouse.lbutton_pressed {
                    let bitem =
                        self.pause_menu
                            .bbox_colliding(self.world_state.cursor);
                    match bitem {
                        Some(item) => {
                            match item.trim().as_ref() {
                                "main_menu" => {
                                    self.screen = GameScreen::Menu
                                }
                                "fullscreen" => {
                                    let fullscreen =
                                        root.is_fullscreen();
                                    root.set_fullscreen(!fullscreen);
                                }
                                "back" => {
                                    self.screen = GameScreen::Game
                                }
                                "exit" => std::process::exit(0),
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
            GameScreen::Menu => {}
            GameScreen::Paused => {
                match key.code {
                    KeyCode::Spacebar => {
                        self.screen = GameScreen::Game
                    }
                    KeyCode::Escape => self.screen = GameScreen::Menu,
                    _ => {}
                }
            }
            GameScreen::Game => {
                match key.code {
                    KeyCode::Spacebar => {
                        self.screen = GameScreen::Paused
                    }
                    KeyCode::Tab => {
                        if self.show_hud {
                            self.show_tools.text = "Menu \u{1B}"
                                .to_string();
                            self.show_hud = false;
                        } else {
                            self.show_tools.text = "Menu \u{1a}"
                                .to_string();
                            self.show_hud = true;
                        }
                    }
                    KeyCode::Char => {
                        match key.printable {
                            '<' => {
                                self.world_state.level =
                                    clamp(self.world_state.level - 1,
                                          self.constants
                                              .highest_world,
                                          0);
                            }
                            '>' => {
                                self.world_state.level =
                                    clamp(self.world_state.level + 1,
                                          self.constants
                                              .highest_world,
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
        .font_type(FontType::Default)
        .renderer(Renderer::OpenGL)
        .font_dimensions(16, 19)
        .font(SHOW_FONT, FontLayout::AsciiInRow)
        .init();

    if TILES {
        unsafe {
            load_custom_font(2);
        }
    }

    let mut game = Game::new(screen_size);
    if DEBUG {
        game.screen = GameScreen::Loading;
        for line in TITLE_CARD.iter() {
            println!("{}", line);
        }
    }

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
