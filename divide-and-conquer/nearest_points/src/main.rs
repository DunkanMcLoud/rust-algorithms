use std::fmt::Pointer;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn get_nearest_pair(points: &[Point]) -> (Point, Point) {
    let sort_by_x = |l: Point, r: Point| l.x < r.x;
    let sort_by_y = |l: Point, r: Point| l.y < r.y;

    let sorted_x = sort(points, &sort_by_x);
    let sorted_y = sort(points, &sort_by_y);

    closest_pair(sorted_x, sorted_y)
}

fn distance(l: &Point, r: &Point) -> u32 {
    (((l.x - r.x).pow(2) + (l.y - r.y).pow(2)) as f64).sqrt() as u32
}

fn closest_pair(l: Vec<Point>, r: Vec<Point>) -> (Point, Point) {
    // split x-sorted and y-sorted arrays into left and right parts

    if l.len() <= 3 {
        return brute_force_find_closest(l.as_slice());
    }

    let median = l[l.len() / 2];
    let (l_x, r_x) = l.iter().partition(|&p| p.x < median.x);
    let (l_y, r_y) = r.iter().partition(|&p| p.x < median.x);
    println!("left X {:?}, right X{:?}", l_x, r_x);
    println!("left Y {:?},right Y{:?}", l_y, r_y);
    let (l1, l2) = closest_pair(l_x, l_y);
    let (r1, r2) = closest_pair(r_x, r_y);

    let nearest_points = {
        if distance(&l1, &l2) < distance(&r1, &r2) {
            (l1, l2)
        } else {
            (r1, r2)
        }
    };

    let delta = f64::min(distance(&l1, &l2) as f64, distance(&r1, &r2) as f64) as u32;

    let strip: Vec<Point> = r
        .iter()
        .cloned()
        .filter(|&p| (p.x - median.x).abs() < delta as i32)
        .collect();

    let closest_in_split = closest_split_pair(&strip, delta);

    match closest_in_split {
        Some(pt) => pt,
        None => nearest_points,
    }
}

fn brute_force_find_closest(points: &[Point]) -> (Point, Point) {
    let (mut p1, mut p2) = (points[0], points[1].clone());
    let mut min_dist = distance(&p1, &p2);

    for i in 0..points.len() {
        for n in (i + 1)..points.len() {
            let d = distance(&points[i], &points[n]);

            if d < min_dist {
                min_dist = d;
                p1 = points[i];
                p2 = points[n];
            }
        }
    }
    (p1, p2)
}

fn closest_split_pair(strip: &[Point], delta: u32) -> Option<(Point, Point)> {
    let mut best_distance = delta;
    let mut best_pair: Option<(Point, Point)> = None;

    assert!(!strip.is_empty());
    for i in 0..strip.len() {
        for j in (i + 1)..strip.len() {
            let d = distance(&strip[i], &strip[j]);
            if d < best_distance {
                best_pair = Some((strip[i], strip[j]));
                best_distance = d;
            }
        }
    }
    best_pair
}

fn sort(points: &[Point], comparator: &dyn Fn(Point, Point) -> bool) -> Vec<Point> {
    if points.len() <= 1 {
        return points.to_vec();
    }

    let (left, right) = points.split_at(points.len() / 2);

    let l_sorted = sort(left, comparator);
    let r_sorted = sort(right, comparator);

    merge(l_sorted.as_slice(), r_sorted.as_slice(), comparator)
}

fn merge(left: &[Point], right: &[Point], compare: &dyn Fn(Point, Point) -> bool) -> Vec<Point> {
    let mut k = 0_usize;
    let mut n = 0_usize;
    let mut res = Vec::new();

    while k < left.len() && n < right.len() {
        if compare(left[k], right[n]) {
            res.push(left[k]);
            k += 1;
        } else {
            res.push(right[n]);
            n += 1;
        }
    }

    while k < left.len() {
        res.push(left[k]);
        k += 1;
    }

    while n < right.len() {
        res.push(right[n]);

        n += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sort_x() {
        let data = vec![(1, 2), (3, 4), (7, 8), (10, 12), (5, 6), (9, 10), (11, 13)]
            .into_iter()
            .map(Point::from)
            .collect::<Vec<Point>>();

        let f = |a: Point, b: Point| a.x < b.x;
        let res = sort(data.as_slice(), &f);

        assert_eq!(
            vec![(1, 2), (3, 4), (5, 6), (7, 8), (9, 10), (10, 12), (11, 13)]
                .into_iter()
                .map(Point::from)
                .collect::<Vec<Point>>(),
            res
        )
    }

    #[test]
    fn test_complex_case() {
        let data = vec![
            (1, 2),
            (3, 4),
            (7, 8),
            (10, 12),
            (5, 6),
            (9, 10),
            (11, 13),
            (20, 21),
            (100, 101),
            (55, 56),
            (39, 40),
            (17, 18),
            (60, 61),
        ];

        let points = get_points(&data);
        let res = get_nearest_pair(&points);

        let expected_pair1 = Point::from((10, 12));
        let expected_pair2 = Point::from((11, 13));

        assert!(
            (res.0 == expected_pair1 && res.1 == expected_pair2)
                || (res.1 == expected_pair1 && res.0 == expected_pair2)
        );
    }

    #[test]
    fn test_1() {
        let data = vec![(1, 2), (3, 4), (7, 8), (10, 12), (5, 6), (9, 10), (11, 13)];

        let points = get_points(&data);

        let res = get_nearest_pair(&points);

        assert_eq!(10, res.0.x);
        assert_eq!(12, res.0.y);
        assert_eq!(11, res.1.x);
        assert_eq!(13, res.1.y);
    }

    fn get_points(tuples: &[(i32, i32)]) -> Vec<Point> {
        tuples.iter().map(|t| Point::from((t.0, t.1))).collect()
    }
}
