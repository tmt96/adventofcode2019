use std::collections::VecDeque;
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

struct IntCodeComputer {
    program: Vec<i32>,
    input: VecDeque<i32>,
    inst_pointer: usize,
}

impl IntCodeComputer {
    fn new(program: &[i32], phase: i32) -> Self {
        Self {
            program: program.to_vec(),
            input: VecDeque::from(vec![phase]),
            inst_pointer: 0,
        }
    }

    fn parse_instruction(&self) -> (i32, Mode, Mode) {
        let inst = self.program[self.inst_pointer];
        let opcode = inst % 100;
        let mut inst = inst / 100;
        let mode_1 = inst % 10;
        inst /= 10;
        let mode_2 = inst % 10;
        (opcode, Mode::from_i32(mode_1), Mode::from_i32(mode_2))
    }

    fn get_val(&self, loc: usize, mode: Mode) -> i32 {
        let res = self.program[loc];
        match mode {
            Mode::Intermediate => res,
            Mode::Position => self.program[res as usize],
        }
    }

    fn run_program(&mut self, input: i32) -> i32 {
        self.input.push_back(input);

        while self.inst_pointer < self.program.len() {
            let (opcode, mode_1, mode_2) = self.parse_instruction();
            match opcode {
                1 => {
                    let (fst, snd, dst) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                        self.program[self.inst_pointer + 3],
                    );
                    self.program[dst as usize] = fst + snd;
                    self.inst_pointer += 4;
                }
                2 => {
                    let (fst, snd, dst) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                        self.program[self.inst_pointer + 3],
                    );
                    self.program[dst as usize] = fst * snd;
                    self.inst_pointer += 4;
                }
                3 => {
                    let dst = self.program[self.inst_pointer + 1];
                    self.program[dst as usize] = self.input.pop_front().unwrap();
                    self.inst_pointer += 2;
                }
                4 => {
                    self.input
                        .push_back(self.get_val(self.inst_pointer + 1, mode_1));
                    self.inst_pointer += 2;
                }
                5 => {
                    let (fst, snd) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                    );
                    self.inst_pointer = if fst != 0 {
                        snd as usize
                    } else {
                        self.inst_pointer + 3
                    }
                }
                6 => {
                    let (fst, snd) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                    );
                    self.inst_pointer = if fst == 0 {
                        snd as usize
                    } else {
                        self.inst_pointer + 3
                    }
                }
                7 => {
                    let (fst, snd, dst) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                        self.program[self.inst_pointer + 3],
                    );
                    self.program[dst as usize] = if fst < snd { 1 } else { 0 };
                    self.inst_pointer += 4;
                }
                8 => {
                    let (fst, snd, dst) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                        self.program[self.inst_pointer + 3],
                    );
                    self.program[dst as usize] = if fst == snd { 1 } else { 0 };
                    self.inst_pointer += 4;
                }
                99 => break,
                opcode => panic!(
                    "Opcode must be 1, 2, 3, 4, 5, 6, 7, 8 or 99, receive {} at {}",
                    opcode, self.inst_pointer,
                ),
            }
        }
        self.input[0]
    }
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
            perm.iter().fold(0, |state, &x| {
                IntCodeComputer::new(input, x).run_program(state)
            })
        })
        .max()
        .unwrap()
}

fn part2(input: &[i32]) -> i32 {
    IntCodeComputer::new(input, 1).run_program(0)
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    // println!("part 2: {}", part2(&input));
    Ok(())
}
