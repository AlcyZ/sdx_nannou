use nannou::prelude::*;

pub mod shapes;

pub fn points_list_diagonals(points: &[Point2]) -> Vec<(Point2, Point2)> {
    let mut points = points.to_owned();

    let len = points.len();
    if len % 2 != 0 {
        points.pop().unwrap();
    }
    let half_len = len / 2;

    (0..half_len)
        .map(|i| (*points.get(i).unwrap(), *points.get(i + half_len).unwrap()))
        .collect()
}
