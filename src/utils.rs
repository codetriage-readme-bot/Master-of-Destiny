extern crate rand;

use std;
use std::cmp;

use worldgen::World;
use worldgen::terrain::{Slope, Tile};
use life;

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
    (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32)
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
    (((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)) as f32)
        .cbrt()
}

pub fn can_move<'a>(map: &'a World, animal: &'a life::Living)
                    -> impl FnMut((i32, i32), (i32, i32)) -> f32 {
    move |from, to| {
        let zloc_from = animal.current_pos().2;
        let uto = (to.0 as usize, to.1 as usize);
        if let Some(new_point) = map.get(uto) {
            let new_zloc = map.location_z_from_to(zloc_from, uto);
            if (zloc_from as i32 - new_zloc as i32).abs() < 2 &&
                !new_point.tiles.borrow()[new_zloc].solid() {
                1.0
            } else {
                0.0
            }
        } else {
            0.0
        }
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
