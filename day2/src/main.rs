use std::fs::read_to_string;
use std::path::Path;

fn run_program(input: &Vec<i32>, fst: i32, snd: i32) -> i32 {
    let mut input = input.to_vec();
    input[1] = fst;
    input[2] = snd;

    let len = input.len();
    for i in (0..len).step_by(4) {
        if i + 3 > len {
            break;
        }
        match input[i] {
            1 => {
                let (fst, snd, dst) = (input[i + 1], input[i + 2], input[i + 3]);
                input[dst as usize] = input[fst as usize] + input[snd as usize]
            }
            2 => {
                let (fst, snd, dst) = (input[i + 1], input[i + 2], input[i + 3]);
                input[dst as usize] = input[fst as usize] * input[snd as usize]
            }
            99 => break,
            _ => panic!("Opcode must be 1, 2 or 99"),
        }
    }
    input[0]
}

fn part1(input: &Vec<i32>) -> i32 {
    run_program(input, 12, 2)
}

fn part2(input: &Vec<i32>, final_res: i32) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(input, noun, verb) == final_res {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/day2.txt");
    let input: Vec<_> = read_to_string(filepath)
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input, 19690720));
    Ok(())
}
