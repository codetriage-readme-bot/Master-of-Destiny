use tcod::RootConsole;
use tcod::colors::*;
use tcod::console::{BackgroundFlag, Console, TextAlignment};

pub trait DrawUI {
    fn draw(&self, mut root: &mut RootConsole, cursor: (i32, i32));
}
pub trait MouseUI {
    fn bbox_colliding(&self, cursor: (i32, i32)) -> Option<String>;
}

type BBox = ((i32, i32), (i32, i32));
pub struct Button {
    pub bbox: BBox,
    pub text: String,
    pub id: String,
}

impl Button {
    fn new(name: &'static str,
           pos: (i32, i32),
           size: (i32, i32))
           -> Button {
        Button {
            bbox: calculate_bbox(pos, size),
            text: name.to_string(),
            id: name.replace(" ", "_").to_lowercase(),
        }
    }
}

impl MouseUI for Button {
    fn bbox_colliding(&self, loc: (i32, i32)) -> Option<String> {
        let (bbs, bbe) = self.bbox;
        println!("{:?} <= {:?} <= {:?}", bbs, loc, bbe);
        if (loc.0 >= bbs.0 && loc.1 <= bbs.1) &&
            (loc.0 <= bbe.0 && loc.1 >= bbe.1)
        {
            Some(self.id.clone())
        } else {
            None
        }
    }
}

impl DrawUI for Button {
    fn draw(&self, mut root: &mut RootConsole, cursor: (i32, i32)) {
        root.set_default_foreground(BLACK);
        if self.bbox_colliding(cursor).is_some() {
            root.set_default_background(WHITE);
        } else {
            root.set_default_background(Color::new(100, 100, 100));
        }
        root.print_ex((self.bbox.0).0,
                      (self.bbox.0).1,
                      BackgroundFlag::Set,
                      TextAlignment::Center,
                      self.text.clone());
        root.set_default_foreground(WHITE);
        root.set_default_background(BLACK);
    }
}

fn calculate_bbox(pos: (i32, i32), size: (i32, i32)) -> BBox {
    (pos, (pos.0 + size.0, pos.1 + size.1))
}

pub struct Layout {
    pub buttons: Vec<Button>,
}

impl Layout {
    pub fn new(elements: Vec<&'static str>,
               pos: (i32, i32),
               button_size: (i32, i32),
               wrap_at: i32)
               -> Layout {
        Layout {
            buttons: elements.iter()
                .enumerate()
                .map(|(i, text)| {
                    let raw_x = i as i32 * button_size.0;
                    Button::new(text,
                                (pos.0 + (raw_x % wrap_at),
                                 pos.1 +
                                 (raw_x / wrap_at *
                                  (button_size.1 + 1))),
                                button_size)
                })
                .collect(),
        }
    }
}

impl DrawUI for Layout {
    fn draw(&self, mut root: &mut RootConsole, cursor: (i32, i32)) {
        for button in self.buttons.iter() {
            button.draw(root, cursor);
        }
    }
}

impl MouseUI for Layout {
    fn bbox_colliding(&self, cursor: (i32, i32)) -> Option<String> {
        for button in self.buttons.iter().rev() {
            if let Some(x) = button.bbox_colliding(cursor) {
                return Some(x);
            }
        }
        None
    }
}
