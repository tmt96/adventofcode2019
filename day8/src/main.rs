use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn counter_map<T: std::cmp::Eq + std::hash::Hash + Copy>(input: &[T]) -> HashMap<T, i32> {
    let mut map = HashMap::new();
    for element in input {
        *map.entry(*element).or_insert(0) += 1;
    }
    map
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i32>> {
    Ok(read_to_string(filepath)?
        .split("")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect())
}

fn part1(input: &[i32], width: usize, height: usize) -> i32 {
    let chunk_size = width * height;
    input
        .chunks(chunk_size)
        .map(counter_map)
        .min_by_key(|map| *map.get(&0).unwrap_or(&0))
        .map(|map| map.get(&1).unwrap_or(&0) * map.get(&2).unwrap_or(&0))
        .unwrap()
}

fn part2(input: &[i32], width: usize, height: usize) -> String {
    let chunk_size = width * height;
    let mut string: String = (0..chunk_size)
        .map(|i| {
            match input
                .iter()
                .skip(i)
                .step_by(chunk_size)
                .skip_while(|&&i| i == 2)
                .next()
            {
                Some(0) => ' ',
                _ => '#',
            }
        })
        .collect();
    (1..height)
        .rev()
        .for_each(|i| string.insert(i * width, '\n'));
    string
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input, 25, 6));
    println!("part 2:\n{}", part2(&input, 25, 6));
    Ok(())
}
