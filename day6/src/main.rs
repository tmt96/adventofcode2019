use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input(filepath: &Path) -> std::io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut orbit_map = HashMap::new();

    reader.lines().filter_map(Result::ok).for_each(|s| {
        let split: Vec<_> = s.split(')').map(|s| s.to_string()).take(2).collect();
        let (fst, snd) = (split[0].to_string(), split[1].to_string());
        orbit_map.entry(fst).or_insert_with(|| vec![]).push(snd);
    });
    Ok(orbit_map)
}

fn part1(orbit_map: &HashMap<String, Vec<String>>) -> i32 {
    fn helper(orbit_map: &HashMap<String, Vec<String>>, planet: &str, count: i32) -> i32 {
        match orbit_map.get(planet) {
            Some(planets) => {
                let sub_obit_count: i32 = planets
                    .iter()
                    .map(|planet| helper(orbit_map, planet, count + 1))
                    .sum();
                sub_obit_count + count
            }
            None => count,
        }
    };
    helper(orbit_map, "COM", 0)
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    // println!("part 2: {}", part2(&modules));
    Ok(())
}
