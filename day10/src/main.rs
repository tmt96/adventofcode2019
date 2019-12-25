use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Position = (i32, i32);

fn difference((row1, col1): Position, (row2, col2): Position) -> Position {
    (row1 - row2, col1 - col2)
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

fn minimize_ratio((row, col): Position) -> Position {
    let gcd = gcd(row, col);
    (row / gcd, col / gcd)
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
                        Some((row as i32, col as i32))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect())
}

fn part1(input: &[Position]) -> i32 {
    input
        .iter()
        .map(|&point| {
            input
                .iter()
                .filter(|&&snd_point| snd_point != point)
                .map(|&snd_point| minimize_ratio(difference(point, snd_point)))
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap() as i32
}

fn part2(input: &[Position]) -> i32 {
    0
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/sample.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
