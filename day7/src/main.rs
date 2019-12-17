use std::fs::read_to_string;
use std::path::Path;

use itertools::Itertools;

enum Mode {
    Position,
    Intermediate,
}

impl Mode {
    fn from_i32(val: i32) -> Self {
        match val {
            0 => Self::Position,
            1 => Self::Intermediate,
            mode => panic!("MOde must be either 0 or 1, receive {}", mode),
        }
    }
}

fn parse_instruction(inst: i32) -> (i32, Mode, Mode) {
    let opcode = inst % 100;
    let mut inst = inst / 100;
    let mode_1 = inst % 10;
    inst /= 10;
    let mode_2 = inst % 10;
    (opcode, Mode::from_i32(mode_1), Mode::from_i32(mode_2))
}

fn get_val(program: &[i32], val: usize, mode: Mode) -> i32 {
    let res = program[val];
    match mode {
        Mode::Intermediate => res,
        Mode::Position => program[res as usize],
    }
}

fn run_program(program: &[i32], input: &[i32]) -> i32 {
    let mut program = program.to_vec();
    let mut input = input.to_vec();
    let mut i = 0;

    while i < program.len() {
        let (opcode, mode_1, mode_2) = parse_instruction(program[i]);
        match opcode {
            1 => {
                let (fst, snd, dst) = (
                    get_val(&program, i + 1, mode_1),
                    get_val(&program, i + 2, mode_2),
                    program[i + 3],
                );
                program[dst as usize] = fst + snd;
                i += 4;
            }
            2 => {
                let (fst, snd, dst) = (
                    get_val(&program, i + 1, mode_1),
                    get_val(&program, i + 2, mode_2),
                    program[i + 3],
                );
                program[dst as usize] = fst * snd;
                i += 4;
            }
            3 => {
                let dst = program[i + 1];
                program[dst as usize] = input.pop().unwrap();
                i += 2;
            }
            4 => {
                input.push(get_val(&program, i + 1, mode_1));
                i += 2;
            }
            5 => {
                let (fst, snd) = (
                    get_val(&program, i + 1, mode_1),
                    get_val(&program, i + 2, mode_2),
                );
                i = if fst != 0 { snd as usize } else { i + 3 }
            }
            6 => {
                let (fst, snd) = (
                    get_val(&program, i + 1, mode_1),
                    get_val(&program, i + 2, mode_2),
                );
                i = if fst == 0 { snd as usize } else { i + 3 }
            }
            7 => {
                let (fst, snd, dst) = (
                    get_val(&program, i + 1, mode_1),
                    get_val(&program, i + 2, mode_2),
                    program[i + 3],
                );
                program[dst as usize] = if fst < snd { 1 } else { 0 };
                i += 4;
            }
            8 => {
                let (fst, snd, dst) = (
                    get_val(&program, i + 1, mode_1),
                    get_val(&program, i + 2, mode_2),
                    program[i + 3],
                );
                program[dst as usize] = if fst == snd { 1 } else { 0 };
                i += 4;
            }
            99 => break,
            opcode => panic!(
                "Opcode must be 1, 2, 3, 4, 5, 6, 7, 8 or 99, receive {} at {}",
                opcode, i,
            ),
        }
    }
    input[0]
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i32>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect())
}

fn part1(input: &[i32]) -> i32 {
    (0..5)
        .permutations(5)
        .map(|perm| {
            perm.iter()
                .fold(0, |state, &x| run_program(input, &[state, x]))
        })
        .max()
        .unwrap()
}

fn part2(input: &[i32]) -> i32 {
    run_program(input, &[5])
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    // println!("part 2: {}", part2(&input));
    Ok(())
}
