use std::cell::RefCell;
use worldgen::{Unit, WorldState, weak_adjacent};
use worldgen::terrain::{State, Tile};

pub mod liquid;
pub mod stone;

pub fn run(ws: &mut WorldState, dt: usize) {
    if dt > 600 {
        if let Some(ref map) = ws.map {
            for y in 0..(ws.map_size.1) {
                let mut my = map[y].borrow_mut();
                for x in 0..(ws.map_size.0) {
                    for height in 0..ws.highest_level {
                        // Basic physics.
                        let adj = weak_adjacent((x, y))
                            .iter()
                            .map(|pnt| {
                                map[pnt.1].borrow()[pnt.0].tiles
                                    [height]
                            })
                            .filter(|tl| *tl != Tile::Empty)
                            .collect::<Vec<_>>();
                        if adj.len() < 2 &&
                            my[x].tiles[height - 1] == Tile::Empty &&
                            my[x].tiles[height + 1] == Tile::Empty
                        {
                            my[x].tiles[height - 1] = my[x].tiles
                                [height];
                            my[x].tiles[height] = Tile::Empty;
                        }
                        /*
                        // More in-depth physics (water-flow, melting)
                        match my[x].tiles[height] {
                            s @ Tile::Stone(_, State::Solid) => {
                                stone::solid_physics(map, s, adj)
                            }
                            s @ Tile::Stone(_, State::Liquid) => {
                                stone::liquid_physics(map, s, adj)
                            }

                            w @ Tile::Water(_, State::Solid, _) => {
                                liquid::solid_physics(map, w, adj)
                            }
                            w @ Tile::Water(_, State::Liquid, _) => {
                                liquid::liquid_physics(map, w, adj)
                            }
                            _ => {
                                /* TODO: Implement physics for other objects */
                            }
                        }
                        */

                    }
                }
            }
        }
    }
}
