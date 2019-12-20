use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;

use itertools::Itertools;

enum ResultCode {
    Output(i32),
    Terminated,
}

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

    fn run_one_turn(&mut self, input: i32) -> ResultCode {
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
                    let output = self.get_val(self.inst_pointer + 1, mode_1);
                    self.inst_pointer += 2;
                    return ResultCode::Output(output);
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
                99 => return ResultCode::Terminated,
                opcode => panic!(
                    "Opcode must be 1, 2, 3, 4, 5, 6, 7, 8 or 99, receive {} at {}",
                    opcode, self.inst_pointer,
                ),
            }
        }
        unreachable!()
    }

    fn run_program(&mut self, input: i32) -> i32 {
        let mut input = input;
        while let ResultCode::Output(i) = self.run_one_turn(input) {
            input = i;
        }
        self.input[0]
    }
}

fn cal_normal_thrust(phases: &[i32], program: &[i32]) -> i32 {
    phases.iter().fold(0, |state, &phase| {
        IntCodeComputer::new(program, phase).run_program(state)
    })
}

fn cal_thurst_with_feedback(phases: &[i32], program: &[i32]) -> i32 {
    let mut computers: Vec<_> = phases
        .iter()
        .map(|&phase| IntCodeComputer::new(program, phase))
        .collect();
    let mut input = 0;
    for i in (0..computers.len()).cycle() {
        match computers[i].run_one_turn(input) {
            ResultCode::Output(i) => input = i,
            ResultCode::Terminated => return input,
        }
    }
    unreachable!()
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
        .map(|perm| cal_normal_thrust(&perm, input))
        .max()
        .unwrap()
}

fn part2(input: &[i32]) -> i32 {
    (5..10)
        .permutations(5)
        .map(|perm| cal_thurst_with_feedback(&perm, input))
        .max()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_test1() {
        let code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let res = part1(&code);
        assert_eq!(43210, res);
    }

    #[test]
    fn day7_test2() {
        let code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let res = part1(&code);
        assert_eq!(54321, res);
    }

    #[test]
    fn day7_test3() {
        let code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let res = part1(&code);
        assert_eq!(65210, res);
    }

    #[test]
    fn day7_test4() {
        let code = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let res = part2(&code);
        assert_eq!(139_629_729, res);
    }

    #[test]
    fn day7_test5() {
        let code = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let res = part2(&code);
        assert_eq!(18216, res);
    }
}
