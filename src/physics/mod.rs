use worldgen::{Unit, WorldState, weak_adjacent};
use worldgen::terrain::{State, Tile};

pub mod liquid;
pub mod stone;

pub fn run(ws: &mut WorldState, dt: usize) {
    if dt > 600 {
        if ws.map.is_some() {
            for y in 0..(ws.map_size.1) {
                for x in 0..(ws.map_size.0) {
                    for height in 0..ws.highest_level {
                        /*              
                        // Basic physics.
                        let adj = weak_adjacent((x, y))
                            .iter()
                            .map(
                                |pnt| map[pnt.1][pnt.0].tiles[height],
                            )
                            .filter(|tl| *tl != Tile::Empty)
                            .collect::<Vec<_>>();
                        if adj.len() < 3 &&
                            map[y][x].tiles[height - 1] ==
                                Tile::Empty
                        {
                            map[y][x].tiles[height - 1] =
                                map[y][x].tiles[height];
                            map[y][x].tiles[height] = Tile::Empty;
                        }
                        // More in-depth physics (water-flow, melting)
                        match map[y][x].tiles[height] {
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
                        }*/

                    }
                }
            }
        }
    }
}
