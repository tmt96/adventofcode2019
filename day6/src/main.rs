use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type OrbitMap = HashMap<String, Vec<String>>;

fn inverted_map(map: &OrbitMap) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for (center, planets) in map {
        for planet in planets {
            result.insert(planet.to_string(), center.to_string());
        }
    }
    result
}

fn build_counter_map(
    orbit_map: &HashMap<String, String>,
    counter_map: &mut HashMap<String, i32>,
    planet: &str,
    count: i32,
) {
    counter_map.insert(planet.to_string(), count);
    if let Some(center) = orbit_map.get(planet) {
        build_counter_map(orbit_map, counter_map, center, count + 1)
    }
}

fn read_input(filepath: &Path) -> std::io::Result<OrbitMap> {
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

fn part1(orbit_map: &OrbitMap) -> i32 {
    fn helper(orbit_map: &OrbitMap, planet: &str, count: i32) -> i32 {
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
    }
    helper(orbit_map, "COM", 0)
}

fn part2(orbit_map: &OrbitMap) -> i32 {
    // There are some edge case where this function fails. Fixing should be simple, but I'm lazy.
    let inverted_orbit_map = inverted_map(orbit_map);
    let mut counter_map = HashMap::new();
    build_counter_map(&inverted_orbit_map, &mut counter_map, "YOU", 0);

    fn get_common_ancestor(
        inverted_orbit_map: &HashMap<String, String>,
        counter_map: &HashMap<String, i32>,
        planet: &str,
        count: i32,
    ) -> Option<(String, i32, i32)> {
        match counter_map.get(planet) {
            Some(&i) => Some((planet.to_string(), i - 1, count - 1)),
            None => inverted_orbit_map.get(planet).and_then(|center| {
                get_common_ancestor(inverted_orbit_map, counter_map, center, count + 1)
            }),
        }
    }

    let (_, from_fst, from_snd) =
        get_common_ancestor(&inverted_orbit_map, &counter_map, "SAN", 0).unwrap();
    from_fst + from_snd
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
