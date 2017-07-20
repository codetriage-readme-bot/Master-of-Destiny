use std::cell::RefCell;
use std::rc::Rc;
use worldgen::{Unit, WorldState, weak_adjacent};
use worldgen::terrain::{State, Tile};

pub mod liquid;
pub mod stone;

pub fn run(ws: &mut WorldState, dt: usize) {
    if dt > 600 {
        if let Some(ref map) = ws.map {
            println!("Map available.");
            for y in 0..(ws.map_size.1) {
                let mut my = map[y].borrow_mut();
                for x in 0..(ws.map_size.0) {
                    let noheight_adj = weak_adjacent((x, y))
                        .iter()
                        .map(|pnt| {
                            println!("Scanning {:?}", pnt);
                            map[pnt.1].borrow()[pnt.0].clone()
                        })
                        .collect::<Vec<_>>();
                    for height in 0..ws.highest_level {
                        println!("Height: {}", height);
                        let adj =
                            noheight_adj.iter()
                                        .map(
                                |unit| unit.tiles[height],
                            )
                                        .filter(|x| *x != Tile::Empty)
                                        .collect::<Vec<_>>();
                        // Basic physics.
                        let mut u = Rc::get_mut(&mut my[x]).unwrap();
                        if adj.len() < 2 &&
                            u.tiles[height - 1] == Tile::Empty &&
                            u.tiles[height + 1] == Tile::Empty
                        {
                            println!("Located insufficiently supported tile, effecting gravity.");
                            u.tiles[height - 1] = u.tiles[height];
                            u.tiles[height] = Tile::Empty;
                        }
                    }
                    // More in-depth physics (water-flow, melting)
                    if let Some(ref changes) =
                        match my[x].tiles[0] {
                            Tile::Stone(_, State::Solid) => {
                                stone::solid_physics((x, y),
                                                     noheight_adj)
                            }
                            Tile::Stone(_, State::Liquid) => {
                                stone::liquid_physics((x, y),
                                                      noheight_adj)
                            }

                            Tile::Water(_, State::Solid, _) => {
                                liquid::solid_physics((x, y),
                                                      noheight_adj)
                            }
                            Tile::Water(_, State::Liquid, _) => {
                                liquid::liquid_physics((x, y),
                                                       noheight_adj)
                            }
                            _ => None,
                        }
                    {

                        // update map
                        for (i, pnt) in weak_adjacent((x, y))
                            .iter()
                            .enumerate()
                        {
                            map[pnt.1].borrow_mut()[pnt.0] =
                                Rc::new(changes[i].clone());
                        }
                    };
                }
            }
        }
    }
}
