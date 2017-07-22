use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use worldgen::{Unit, WorldState, strict_adjacent};
use worldgen::terrain::{State, Tile};

pub mod liquid;
pub mod stone;

pub trait PhysicsActor {
    fn solid(&self) -> bool;
    fn heavy(&self) -> bool;
}

fn unsupported(tile: Tile,
               adj: Vec<Tile>,
               above: Tile,
               below: Tile)
               -> bool {
    let solid_cnt = adj.iter()
        .take_while(|x| x.solid())
        .count();
    !below.solid() && (!above.solid() || !tile.heavy()) &&
        solid_cnt < 2
}
pub fn run(ws: &mut WorldState, dt: usize) {
    if let Some(ref world) = ws.map {
        let map = &world.map;
        for y in 0..(ws.map_size.1) {
            for x in 0..(ws.map_size.0) {
                let unit = &map[y][x];
                for h in 0..unit.tiles.len() {
                    let tile = unit.tiles[h];
                    let adj = strict_adjacent((x, y))
                        .iter()
                        .map(|pnt| map[pnt.1][pnt.0].tiles[h])
                        .collect::<Vec<_>>();
                    if unsupported(tile,
                                   adj,
                                   *unit.tiles
                                   .get(h + 1)
                                   .unwrap_or(&Tile::Empty),
                                   *unit.tiles
                                   .get(h - 1)
                                   .unwrap_or(&Tile::Empty))
                    {
                        println!("Found unsupported tile at {:?}",
                                 (x, y));
                    }
                }
            }
        }
    }
}
