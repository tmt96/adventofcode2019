use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(module: &Vec<i32>) -> i32 {
    module.iter().map(|i| max(i / 3 - 2, 0)).sum()
}

fn part2(module: &Vec<i32>) -> i32 {
    let transform_fn = |i: &i32| -> i32 {
        let (mut i, mut result) = (*i, 0);
        while i > 0 {
            i = max(i / 3 - 2, 0);
            result += i;
        }
        result
    };

    module.iter().map(transform_fn).sum()
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/day1.txt");
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let modules: Vec<_> = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("part 1: {}", part1(&modules));
    println!("part 2: {}", part2(&modules));
    Ok(())
}
