use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use worldgen::{Unit, WorldState, weak_adjacent};
use worldgen::terrain::{State, Tile};

pub mod liquid;
pub mod stone;

pub fn run(ws: &mut WorldState, dt: usize) {
    if let Some(ref map) = ws.map {
        for y in 0..(ws.map_size.1) {
            for x in 0..(ws.map_size.0) {
                let noheight_adj = weak_adjacent((x, y))
                    .iter()
                    .map(|pnt| map[pnt.1].borrow()[pnt.0].clone())
                    .collect::<Vec<Rc<_>>>();
                let my = map[y].borrow();
                for height in 0..ws.highest_level {
                    let adj =
                        noheight_adj.iter()
                        .map(
                            |unit| unit.tiles.borrow()[height],
                        )
                        .filter(|x| *x != Tile::Empty)
                        .collect::<Vec<_>>();
                    // Basic physics.
                    let u = &my[x];
                    let mut ut = u.tiles.borrow_mut();
                    if adj.len() < 2 &&
                        ut[height - 1] == Tile::Empty &&
                        *ut.get(height + 1)
                        .unwrap_or(&Tile::Empty) ==
                        Tile::Empty
                    {
                        ut[height - 1] = ut[height];
                        ut[height] = Tile::Empty;
                    }
                }
                // More in-depth physics (water-flow, melting)
                if let Some(ref changes) =
                    match my[x].tiles.borrow()[0] {
                        Tile::Stone(_, State::Solid) => {
                            stone::solid_physics((x, y), noheight_adj)
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
