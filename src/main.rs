#![feature(box_syntax,
           box_patterns,
           vec_remove_item,
           conservative_impl_trait,
           exclusive_range_pattern,
           const_fn)]
#![allow(dead_code)]

extern crate tcod;
extern crate tcod_sys;
#[macro_use]
extern crate pipeline;

#[macro_export]
macro_rules! matches {
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
}

#[macro_export]
macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

use tcod::{Color, FontLayout, FontType, Renderer, RootConsole};
use tcod::console::{BackgroundFlag, Console, TextAlignment};
use tcod::input;
use tcod::input::KeyCode;

mod utils;
use utils::{Rect2D, clamp};

mod life;

mod draw;
use draw::draw_map;

mod physics;

#[macro_use]
mod ui;
use ui::{Button, DrawUI, Layout, MouseUI, Textbox};

mod worldgen;
use worldgen::{World, WorldState};
use worldgen::terrain::{BASE, TILES};

mod time;

const SHOW_FONT: &'static str = "assets/master20x20_ro.png";

const SHOW_SIZE: (i32, i32) = (75, 50);
const MAP_SIZE: (usize, usize) = (110, 70);

const MOVE_DIST: i32 = 5;

enum GameScreen {
    Menu,
    SelectCommand,
    SelectArea,
    SelectBuildable,
    SelectDiggable,
    SelectOtherCommand,
    Game,
    GetSeed,
    Loading,
    Paused,
}

struct Calc {
    pub max_screen_move: (i32, i32),
    pub highest_world: i32,
    pub screen_size: (i32, i32),
}

enum PartialCommand {
    Move,
    Other,
    Build,
    Dig,
    Destroy,
    Cancel,
}

struct Game {
    show_hud: bool,
    constants: Calc,
    last_time: usize,
    menu: Layout,
    command_list: Layout,
    pause_menu: Layout,
    build_commands: Layout,
    dig_commands: Layout,
    other_commands: Layout,
    move_commands: Layout,
    textbox: Textbox,
    selection: Rect2D,
    show_tools: Button,
    partial_command: PartialCommand,
    seed: u32,
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
            partial_command: PartialCommand::Cancel,
            selection: ((0, 0), (0, 0)),
            last_time: 0,
            time: 0,
            screen: GameScreen::Menu,
            show_tools: Button::new("Menu \u{1A}",
                                    (screen_size.0 - 7,
                                     screen_size.1 - 2),
                                    (6, 1)),
            menu: Layout::new(vec!["New Game", "Use Seed", "Exit"],
                              (screen_size.0 / 2, 15),
                              (8, 0),
                              8),

            build_commands: Layout::new(vec!["Wall", "Fence",
                                             "Ramp"],
                                        (screen_size.0 / 2, 15),
                                        (8, 0),
                                        8),
            dig_commands: Layout::new(vec!["Mine"],
                                      (screen_size.0 / 2, 15),
                                      (8, 0),
                                      8),
            move_commands: Layout::new(vec!["Go To", "Cart Goods"],
                                       (screen_size.0 / 2, 15),
                                       (10, 0),
                                       10),
            other_commands: Layout::new(vec!["Gather Plants",
                                             "Fell Trees"],
                                        (screen_size.0 / 2, 15),
                                        (13, 0),
                                        13),

            command_list: Layout::new(vec!["Build", "Destroy",
                                           "Dig", "Move", "Cancel",
                                           "Other", "Back"],
                                      (screen_size.0 / 2, 15),
                                      (8, 0),
                                      8),
            pause_menu: Layout::new(vec!["Main Menu",
                                         "Back",
                                         "Fullscreen",
                                         "Exit"],
                                    (screen_size.0 / 2, 4),
                                    (10, 0),
                                    10),
            textbox: Textbox::new("Seed",
                                  (screen_size.0 / 2, 30),
                                  (10, 0)),
            world_state: WorldState::new(),
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
            GameScreen::SelectArea => {
                root.clear();
                draw_map(root,
                         &self.world_state,
                         false,
                         self.last_time);
                let ((sx1, sy1), (sx2, sy2)) = self.selection;
                for y in sy1..sy2 {
                    for x in sx1..sx2 {
                        root.set_char_foreground(x as i32,
                                                 y as i32,
                                                 Color::new(100,
                                                            100,
                                                            100));
                    }
                }
            }
            GameScreen::SelectDiggable => {
                root.clear();
                self.dig_commands
                    .draw(root, self.world_state.cursor);
            }
            GameScreen::SelectBuildable => {
                root.clear();
                self.build_commands
                    .draw(root, self.world_state.cursor);
            }
            GameScreen::SelectOtherCommand => {
                root.clear();
                self.other_commands
                    .draw(root, self.world_state.cursor);
            }

            GameScreen::SelectCommand => {
                root.clear();
                self.command_list
                    .draw(root, self.world_state.cursor);
            }
            GameScreen::GetSeed => {
                root.clear();
                self.textbox
                    .draw(root, self.world_state.cursor);
            }
            GameScreen::Loading => {
                root.clear();
                root.print_ex(self.constants.screen_size.0 / 2,
                              2,
                              BackgroundFlag::Set,
                              TextAlignment::Center,
                              "Loading...");
                for c in 0..(16 * 19) {
                    if let Some(chr) = std::char::from_u32(c as u32) {
                        root.put_char(c %
                                          self.constants
                                              .screen_size
                                              .0,
                                      c /
                                          self.constants
                                              .screen_size
                                              .0 +
                                          3,
                                      chr,
                                      BackgroundFlag::Set);
                    }
                }
            }
            GameScreen::Menu => {
                root.clear();
                let TITLE_CARD: Vec<&'static str> = r" _      ____  ____ _____ _____ ____    ____  _____
/ \__/|/  _ \/ ___Y__ __Y  __//  __\  /  _ \/    /
| |\/||| / \||    \ / \ |  \  |  \/|  | / \||  __\
| |  ||| |-||\___ | | | |  /_ |    /  | \_/|| |
\_/  \|\_/ \|\____/ \_/ \____\\_/\_\  \____/\_/

 ____  _____ ____ _____ _  _     ___  _
/  _ \/  __// ___Y__ __Y \/ \  /|\  \//
| | \||  \  |    \ / \ | || |\ || \  /
| |_/||  /_ \___ | | | | || | \|| / /
\____/\____\\____/ \_/ \_/\_/  \|/_/"
                    .split("\n")
                    .collect::<Vec<_>>();
                for (i, line) in TITLE_CARD.iter().enumerate() {
                    root.print_ex(1,
                                  i as i32 + 2,
                                  BackgroundFlag::None,
                                  TextAlignment::Left,
                                  line.to_string());
                }

                self.menu
                    .draw(root, self.world_state.cursor);
            }
            GameScreen::Game => {
                // TODO: find an order where the draw queue from the
                // animal-thread doesn't get mixed with the update
                // queue.
                let current_time = time::get_world_time();
                draw_map(root,
                         &self.world_state,
                         self.show_hud,
                         self.last_time);
                self.world_state.update(current_time,
                                        current_time -
                                            self.last_time);
                self.last_time = current_time;
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
            GameScreen::SelectArea => {
                let (point1, point2) = self.selection;
                if mouse.lbutton_pressed {
                    let p1_unset = point1 == (0, 0);
                    let p2_unset = point2 == (0, 0);
                    let mouse_pos =
                        (self.world_state.cursor.0 as usize,
                         self.world_state.cursor.1 as usize);

                    if p1_unset {
                        self.selection = (mouse_pos, point2);
                    } else if p2_unset {
                        self.selection = (point1, mouse_pos);
                    } else if !p2_unset && !p1_unset {
                        use life::Order::*;
                        if let Some(order) =
                            self.world_state.commands.pop()
                        {
                            let new = match order {
                                Mine(_) => Mine(self.selection),
                                BuildWall(_) => {
                                    BuildWall(self.selection)
                                }
                                BuildFence(_) => {
                                    BuildFence(self.selection)
                                }
                                BuildRamp(..) => {
                                    BuildRamp(self.selection.0)
                                }
                                Go(_) => {
                                    let (x, y) = self.selection.0;
                                    if let Some(ref ws) =
                                        self.world_state.map
                                    {
                                        Go((x,
                                            y,
                                            ws.map
                                              .location_z((x, y))))
                                    } else {
                                        Go((x, y, 100))
                                    }
                                }
                                CartGoods(_) => {
                                    CartGoods(self.selection)
                                }
                                GatherPlants(_) => {
                                    GatherPlants(self.selection)
                                }
                                FellTrees(_) => {
                                    FellTrees(self.selection)
                                }
                            };
                            println!("{:?}", new);
                            self.world_state.commands.push(new);
                        }
                        self.screen = GameScreen::Game;
                    }
                } else if mouse.rbutton_pressed {
                    self.selection = ((0, 0), (0, 0));
                }
                //self.selection = ((0, 0), (0, 0));
            }

            GameScreen::SelectBuildable => {
                menu_event!{
                    (mouse, self.build_commands)
                    "wall" => {
                        self.world_state.commands.push(
                            life::Order::BuildWall(((0,0), (0,0)))
                        );
                    }
                    "fence" => {
                        self.world_state.commands.push(
                            life::Order::BuildFence(((0,0), (0,0)))
                        );
                    }
                    "ramp" => {
                        self.world_state.commands.push(
                            life::Order::BuildRamp((0,0))
                        );
                    }
                }
                self.selection = ((0, 0), (0, 0));
                self.screen = GameScreen::SelectArea;
            }
            GameScreen::SelectOtherCommand => {
                menu_event!{
                    (mouse, self.other_commands)
                    "gather_plants" => {
                        self.world_state.commands.push(
                            life::Order::GatherPlants(((0,0), (0,0)))
                        );
                    }
                    "fell_trees" => {
                        self.world_state.commands.push(
                            life::Order::FellTrees(((0,0), (0,0)))
                        );
                    }
                }
                self.selection = ((0, 0), (0, 0));
                self.screen = GameScreen::SelectArea;
            }
            GameScreen::SelectDiggable => {
                menu_event!{
                    (mouse, self.other_commands)
                    "mine" => {
                        self.world_state.commands.push(
                            life::Order::Mine(((0,0), (0,0)))
                        );
                    }
                }
                self.selection = ((0, 0), (0, 0));
                self.screen = GameScreen::SelectArea;
            }

            GameScreen::Menu => {
                menu_event!{
                    (mouse, self.menu)
                    "new_game" => { self.init_game(None) }
                    "use_seed" => {
                        self.screen = GameScreen::GetSeed;
                    }
                    "resume" => {
                        self.screen = GameScreen::Game
                    }
                    "exit" => { std::process::exit(0) }
                }
            }
            GameScreen::SelectCommand => {
                self.selection = ((0, 0), (0, 0));
                menu_event!{
                    (mouse, self.command_list)
                    "build" => {
                        self.partial_command =
                            PartialCommand::Build;
                        self.screen =
                            GameScreen::SelectBuildable;
                    }
                    "destroy" => {
                        self.partial_command =
                            PartialCommand::Destroy;
                        self.screen =
                            GameScreen::SelectArea;
                    }
                    "dig" => {
                        self.partial_command =
                            PartialCommand::Dig;
                        self.screen =
                            GameScreen::SelectDiggable;
                    }
                    "move" => {
                        self.partial_command =
                            PartialCommand::Move;
                        self.screen =
                            GameScreen::SelectArea;
                    }
                    "other" => {
                        self.partial_command =
                            PartialCommand::Other;
                        self.screen = GameScreen::SelectOtherCommand;
                    }
                    "cancel" => {
                        self.partial_command =
                            PartialCommand::Cancel;
                        self.screen =
                            GameScreen::SelectArea;
                    }
                    "back" => {
                        self.screen = GameScreen::Game;
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
                menu_event!{
                    (mouse, self.menu)
                    "main_menu" => {
                        self.screen = GameScreen::Menu
                    }
                    "fullscreen" => {
                        let fullscreen =
                            root.is_fullscreen();
                        root.set_fullscreen(!fullscreen);
                    }
                    "back" => {
                        self.screen = GameScreen::Game;
                    }
                    "exit" => {
                        std::process::exit(0)
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, key: &input::Key) {
        match self.screen {
            GameScreen::SelectOtherCommand => {}
            GameScreen::SelectCommand => {}
            GameScreen::SelectArea => {}
            GameScreen::SelectDiggable => {}
            GameScreen::SelectBuildable => {}

            GameScreen::GetSeed => {
                if key.code != input::KeyCode::Enter {
                    self.textbox.input(key);
                } else {
                    if let Ok(s) = self.textbox.value.parse::<u32>() {
                        self.init_game(Some(s));
                    }
                }
            }
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
                            'c' => {
                                self.screen =
                                    GameScreen::SelectCommand;
                            }
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
        .title("Master of Destiny")
        .font_dimensions(16, 20)
        .font_type(FontType::Default)
        .renderer(Renderer::OpenGL)
        .font(SHOW_FONT, FontLayout::AsciiInRow)
        .init();

    if TILES {
        let mut x = 0;
        for chr in BASE..(BASE + 47) {
            root.map_ascii_code_to_font(chr as i32,
                                        x % 16,
                                        (x / 16) as i32 + 16);
            x += 1;
        }
    }

    let mut game = Game::new(screen_size);

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
