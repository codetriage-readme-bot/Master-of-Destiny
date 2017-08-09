use utils::strict_adjacent;
use worldgen::{World, WorldState};
use worldgen::terrain::Tile;

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

/*macro_rules! get(
($e:expr) => (match $e { Some(e) => e, None => return None })
);*/

pub fn run(ws: &mut WorldState, dt: usize) {
    /*
    if let Some(ref world) = ws.map {
    let map = &world.map;
    for y in 0..(world.map_size.1) {
    for x in 0..(world.map_size.0) {
    let unit = &map[y][x];
    let mut ut = unit.tiles.borrow_mut();
    for h in 0..ut.len() {
    let tile = ut[h];

    let adj = strict_adjacent((x, y))
    .iter()
    .map(|pnt| {
    let unit = get!(world.get(*pnt));
    let tiles = unit.tiles.borrow();
    Some(tiles[h])
})
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<_>>();
    if unsupported(tile,
    adj,
     *ut.get(h + 1)
    .unwrap_or(&Tile::Empty),
     *ut.get((h.checked_sub(1)
    .unwrap_or(0)))
    .unwrap_or(&Tile::Empty))
    {
    let tmp = ut[h];
    ut[h] = Tile::Empty;
    ut[h - 1] = tmp;
}
}
}
}
}
     */
}
