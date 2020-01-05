use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use itertools::Itertools;

type Position = (i32, i32);

fn difference((col1, row1): Position, (col2, row2): Position) -> Position {
    (col1 - col2, row1 - row2)
}

fn manhattan_len((col, row): Position) -> i32 {
    col.abs() + row.abs()
}

fn gcd(a: i32, b: i32) -> i32 {
    let (a, b) = (a.abs(), b.abs());
    match (a, b) {
        (a, 0) => a,
        (0, b) => b,
        (a, b) if a < b => gcd(b, a),
        (a, b) => gcd(b, a % b),
    }
}

fn minimize_ratio((col, row): Position) -> Position {
    let gcd = gcd(col, row);
    (col / gcd, row / gcd)
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<Position>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .flat_map(|(row, line)| -> Vec<_> {
            line.chars()
                .enumerate()
                .filter_map(move |(col, ch)| {
                    if ch == '#' {
                        Some((col as i32, row as i32))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect())
}

fn gun_loc(input: &[Position]) -> (Position, i32) {
    input
        .iter()
        .map(|&point| {
            let asteroid_detected_count = input
                .iter()
                .filter(|&&snd_point| snd_point != point)
                .map(|&snd_point| minimize_ratio(difference(point, snd_point)))
                .collect::<HashSet<_>>()
                .len();
            (point, asteroid_detected_count as i32)
        })
        .max_by_key(|(_, count)| *count)
        .unwrap()
}

fn part1(input: &[Position]) -> i32 {
    gun_loc(input).1
}

fn part2(input: &[Position]) -> i32 {
    let loc = gun_loc(input).0;
    println!("loc: col: {}, row: {}", loc.0, loc.1);
    let mut positions: Vec<_> = input
        .to_vec()
        .into_iter()
        .filter(|&snd_point| snd_point != loc)
        .collect();
    positions.sort_by_key(|&pos| manhattan_len(difference(pos, loc)));

    let map = positions
        .iter()
        .map(|&point| (minimize_ratio(difference(point, loc)), point))
        .into_group_map();
    let mut angle_points_list: Vec<_> = map
        .into_iter()
        .map(|((col, row), point)| ((col as f64).atan2(row as f64), point))
        .collect();
    angle_points_list.sort_by(|(angle1, _), (angle2, _)| angle2.partial_cmp(angle1).unwrap());

    let mut planet_iters: Vec<_> = angle_points_list
        .into_iter()
        .map(|(_, points)| points.into_iter())
        .collect();

    let mut counter = 0;
    for i in (0..planet_iters.len()).cycle() {
        if let Some((col, row)) = planet_iters[i].next() {
            println!("col: {}, row: {}", col, row);
            counter += 1;
            if counter == 200 {
                return col * 100 + row;
            }
        }
    }
    unreachable!()
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
