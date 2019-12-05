use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::ops::Add;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
type Wire = Vec<Point>;

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn intersection(segment1: &[Point], segment2: &[Point]) -> Option<Point> {
    let (point1, point2) = (segment1[0], segment1[1]);
    let (point3, point4) = (segment2[0], segment2[1]);
    let intersected = (point1.x - point3.x) * (point2.x - point4.x) < 0
        && (point1.y - point3.y) * (point2.y - point4.y) < 0;

    if intersected {
        let mut points = [point1, point2, point3, point4];
        points.sort_by_key(|p| p.x);
        let x = points[2].x;
        points.sort_by_key(|p| p.y);
        let y = points[2].y;
        Some(Point { x, y })
    } else {
        None
    }
}

fn process_segment(segment: &str) -> Point {
    let (dir, len) = segment.split_at(1);
    let len: i32 = len.parse().unwrap();
    match dir {
        "R" => Point { x: len, y: 0 },
        "L" => Point { x: -len, y: 0 },
        "U" => Point { x: 0, y: len },
        "D" => Point { x: 0, y: -len },
        _ => panic!("invalid direction"),
    }
}

fn process_line(path: &str) -> Wire {
    let points = path
        .split(',')
        .scan(Point { x: 0, y: 0 }, |cur_point, segment| {
            *cur_point = *cur_point + process_segment(segment);
            Some(*cur_point)
        });
    once(Point { x: 0, y: 0 }).chain(points).collect()
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<Wire>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .map(|s| process_line(&s))
        .collect())
}

fn part1(input: &[Wire]) -> i32 {
    let (wire1, wire2) = (&input[0], &input[1]);
    wire2
        .windows(2)
        .flat_map(|segment2| {
            wire1
                .windows(2)
                .filter_map(move |segment1| intersection(segment1, segment2))
                .map(|p| p.x.abs() + p.y.abs())
        })
        .min()
        .unwrap_or(0)
}

fn part2(input: &[Wire]) -> i32 {
    let (wire1, wire2) = (&input[0], &input[1]);
    wire2
        .windows(2)
        .scan(0, |state, segment2| {
            let (mut wire1_len, mut result) = (0, 0);
            for segment1 in wire1.windows(2) {
                if let Some(p) = intersection(segment1, segment2) {
                    result = *state + wire1_len + segment2[0].distance(p) + segment1[0].distance(p);
                    break;
                } else {
                    wire1_len += segment1[0].distance(segment1[1])
                }
            }
            *state += segment2[0].distance(segment2[1]);
            Some(result)
        })
        .filter(|&i| i > 0)
        .min()
        .unwrap_or(0)
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
