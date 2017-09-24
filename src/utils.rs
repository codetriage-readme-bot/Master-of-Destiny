extern crate rand;

use std;
use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use life;
use worldgen::World;

use physics::PhysicsActor;

use self::rand::Rng;

pub type Point2D = (usize, usize);
pub type Point3D = (usize, usize, usize);
pub type Rect2D3D = ((usize, usize), (usize, usize), usize);
pub type Rect2D = ((usize, usize), (usize, usize));

/// Returns the coordinates adjacent to the given point.
pub fn strict_adjacent((x, y): (usize, usize))
    -> Vec<(usize, usize)> {
    vec![(x + 1, y),
         (x.checked_sub(1).unwrap_or(0), y),
         (x, y.checked_sub(1).unwrap_or(0)),
         (x, y + 1)]
}

/// Returns the coordinates adjacent to the given point, and the point itself.
pub fn weak_adjacent(p: Point2D) -> Vec<Point2D> {
    let mut v = strict_adjacent(p);
    v.push(p);
    v
}

pub fn distance((x1, y1): Point2D, (x2, y2): Point2D) -> f32 {
    (((x2 as i32 - x1 as i32).pow(2) +
          (y2 as i32 - y1 as i32).pow(2)) as f32)
    .sqrt()
}

/// Make sure a value is between two others. Values are required to be Ord.
pub fn clamp<T: Ord>(value: T, max: T, min: T) -> T {
    cmp::min(max, cmp::max(min, value))
}

pub fn nearest_perimeter_point(((x1, y1), (x2, y2), _): Rect2D3D,
                               (x, y, z): Point3D)
    -> Point3D {
    let x = clamp(x, x1, x2);
    let y = clamp(y, y1, y2);

    let dl = ((x - x1) as i32).abs();
    let dr = ((x - x2) as i32).abs();
    let dt = ((y - y1) as i32).abs();
    let db = ((y - y2) as i32).abs();

    let m = vec![dl, dr, dt, db]
        .iter()
        .fold(std::i32::MAX, |x, y| std::cmp::min(x, *y));
    if m == dt {
        (x, y1, z)
    } else if m == db {
        (x, y2, z)
    } else if m == dl {
        (x1, y, z)
    } else {
        (x2, y, z)
    }
}

pub fn distance3_d((x1, y1, z1): Point3D,
                   (x2, y2, z2): Point3D)
    -> f32 {
    let x1 = x1 as isize;
    let y1 = y1 as isize;
    let z1 = z1 as isize;
    let x2 = x2 as isize;
    let y2 = y2 as isize;
    let z2 = z2 as isize;
    (((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)) as f32)
        .cbrt()
}

pub fn can_move<'a>(map: &'a World,
                    animal: &'a life::Living,
                    to: (usize, usize))
    -> bool {
    let zloc_from = animal.current_pos().2;
    let uto = (to.0 as usize, to.1 as usize);
    if let Some(new_point) = map.get(uto) {
        let new_zloc = map.location_z_from_to(zloc_from, uto);
        if (zloc_from as i32 - new_zloc as i32)
            .abs() < 2 &&
            !new_point.tiles.borrow()[new_zloc]
                .solid()
        {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn random_point(min_x: usize,
                    max_x: usize,
                    min_y: usize,
                    max_y: usize)
    -> Point2D {
    let mut trng = rand::thread_rng();
    (trng.gen_range(min_x, max_x), trng.gen_range(min_y, max_y))
}

pub fn strict_3d_adjacent(pos: Point3D, map: &World) -> Vec<Point3D> {
    strict_adjacent((pos.0, pos.1))
        .iter()
        .map(|pnt| {
            (pnt.0, pnt.1, map.location_z_from_to(pos.2, *pnt))
        })
        .filter(|pnt3d| {
            map.get((pnt3d.0, pnt3d.1))
               .map_or(false, |unit| {
                unit.tiles
                    .borrow()
                    .get(pnt3d.2)
                    .map_or(false, |x| !x.solid())
            })
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    pos: Point3D,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Greedy best-first pathfinding search. Works on 3D points, but
/// treats them as just levels of a 2D plane, not full 3D. This means
/// that when it goes from one tile to the next, it chooses the
/// closest empty tile on the second one. I chose GBFS because it is
/// fast and simple, and finding the absolute best path is not a goal.
pub fn find_path<'a>(map: &World,
                     start: Point3D,
                     goal: Point3D,
                     can_move: Box<Fn(Point3D) -> bool + 'a>)
    -> Option<Vec<Point3D>> {
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
                      cost: 0,
                      pos: start,
                  });

    let mut came_from = HashMap::new();
    came_from.insert(start, None);

    while frontier.len() != 0 {
        let current = frontier.pop();
        if current.unwrap().pos == goal {
            break;
        }
        for next in strict_3d_adjacent(current.unwrap().pos, map) {
            if !came_from.contains_key(&next) && can_move(next) {
                frontier.push(State {
                                  pos: next,
                                  cost: distance3_d(goal, next) as
                                      isize,
                              });
                came_from.insert(next, current.map(|a| a.pos));
            }
        }
    }

    let mut current = goal;
    let mut path = vec![current];
    while current != start {
        if let Some(c) = came_from.get(&current) {
            if let Some(c) = *c {
                current = c;
                path.push(current);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    path.push(start);
    path.reverse();
    Some(path)
}
