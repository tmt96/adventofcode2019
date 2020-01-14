use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type ChemInput = (String, i64);
type ChemMap = HashMap<String, (i64, Vec<ChemInput>)>;

const ORE: &str = "ORE";
const FUEL: &str = "FUEL";

fn process_line(inst: &str) -> (String, (i64, Vec<ChemInput>)) {
    let process_chem = |s: &str| -> ChemInput {
        let split_str: Vec<_> = s.trim().split_whitespace().collect();
        (
            split_str[1].to_string(),
            split_str[0].parse::<i64>().unwrap(),
        )
    };

    let split_str: Vec<_> = inst.split("=>").collect();
    let (input, output) = (split_str[0], split_str[1]);
    let processed_input: Vec<_> = input.split(',').map(process_chem).collect();
    let (output_name, output_count) = process_chem(output);
    (output_name, (output_count, processed_input))
}

fn find_required_ore(
    chem_map: &ChemMap,
    chem: &str,
    count: i64,
    leftover: &mut HashMap<String, i64>,
) -> i64 {
    let leftover_count = *leftover.get(chem).unwrap_or(&0);
    let count = count - leftover_count;
    leftover.insert(chem.to_string(), 0.max(-count));

    if count <= 0 {
        0
    } else if chem == ORE {
        count
    } else {
        let (batch_size, input_list) = &chem_map[chem];
        let batch_count = (count + batch_size - 1) / batch_size;
        leftover
            .entry(chem.to_string())
            .and_modify(|leftover| *leftover += batch_size * batch_count - count);

        input_list
            .iter()
            .map(|(input, input_count)| {
                find_required_ore(chem_map, input, *input_count * batch_count, leftover)
            })
            .sum()
    }
}

fn find_required_ore_for_fuel(input: &ChemMap, count: i64) -> i64 {
    let mut leftover_map = HashMap::new();
    find_required_ore(input, FUEL, count, &mut leftover_map)
}

fn read_input(filepath: &Path) -> std::io::Result<ChemMap> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .map(|s| process_line(&s))
        .collect())
}

fn part1(input: &ChemMap) -> i64 {
    find_required_ore_for_fuel(input, 1)
}

fn part2(input: &ChemMap) -> i64 {
    const ORE_COUNT: i64 = 1_000_000_000_000;
    let mut min_fuel = ORE_COUNT / find_required_ore_for_fuel(input, 1);
    let mut max_fuel = min_fuel * 2;
    while max_fuel - min_fuel > 1 {
        let mid = (min_fuel + max_fuel) / 2;
        if find_required_ore_for_fuel(input, mid) > ORE_COUNT {
            max_fuel = mid;
        } else {
            min_fuel = mid
        }
    }
    min_fuel
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
