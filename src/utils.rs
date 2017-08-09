use std;
use std::cmp;

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
