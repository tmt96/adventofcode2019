use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;

trait AutoExpand {
    type Item;
    fn expandable_get(&mut self, pos: usize) -> &Self::Item;
    fn expandable_set(&mut self, pos: usize, item: Self::Item);
}

impl<T: Default + Clone> AutoExpand for Vec<T> {
    type Item = T;

    fn expandable_get(&mut self, pos: usize) -> &Self::Item {
        let len = self.len();
        if pos + 1 > len {
            self.extend(vec![T::default(); pos + 1 - len].into_iter());
        }
        &self[pos]
    }

    fn expandable_set(&mut self, pos: usize, item: Self::Item) {
        let len = self.len();
        if pos + 1 > len {
            self.extend(vec![T::default(); pos + 1 - len].into_iter());
        }
        self[pos] = item;
    }
}

enum ResultCode {
    Output(i64),
    Terminated,
}

enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn from_i64(val: i64) -> Self {
        match val {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            mode => panic!("Mode must be either 0, 1 or 2, receive {}", mode),
        }
    }
}

struct IntCodeComputer {
    program: Vec<i64>,
    input: Vec<i64>,
    inst_pointer: usize,
    relative_base: i64,
}

impl IntCodeComputer {
    fn new(program: &[i64]) -> Self {
        Self {
            program: program.to_vec(),
            input: Vec::new(),
            inst_pointer: 0,
            relative_base: 0,
        }
    }

    fn parse_instruction(&self) -> (i64, Mode, Mode, Mode) {
        let inst = self.program[self.inst_pointer];
        let opcode = inst % 100;
        let mut inst = inst / 100;
        let mode_1 = inst % 10;
        inst /= 10;
        let mode_2 = inst % 10;
        inst /= 10;
        let mode_3 = inst % 10;
        (
            opcode,
            Mode::from_i64(mode_1),
            Mode::from_i64(mode_2),
            Mode::from_i64(mode_3),
        )
    }

    fn get_val(&mut self, loc: usize, mode: Mode) -> i64 {
        let res = *self.program.expandable_get(loc);
        match mode {
            Mode::Immediate => res,
            Mode::Position => *self.program.expandable_get(res as usize),
            Mode::Relative => *self
                .program
                .expandable_get((res + self.relative_base) as usize),
        }
    }

    fn set_val(&mut self, loc: usize, val: i64, mode: Mode) {
        let res = *self.program.expandable_get(loc);
        match mode {
            Mode::Position => self.program.expandable_set(res as usize, val),
            Mode::Relative => self
                .program
                .expandable_set((res + self.relative_base) as usize, val),
            _ => panic!(),
        }
    }

    fn run_one_turn(&mut self, input: i64) -> ResultCode {
        self.input.push(input);
        while self.inst_pointer < self.program.len() {
            let (opcode, mode_1, mode_2, mode_3) = self.parse_instruction();
            match opcode {
                1 => {
                    let (fst, snd) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                    );
                    self.set_val(self.inst_pointer + 3, fst + snd, mode_3);
                    self.inst_pointer += 4;
                }
                2 => {
                    let (fst, snd) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                    );
                    self.set_val(self.inst_pointer + 3, fst * snd, mode_3);
                    self.inst_pointer += 4;
                }
                3 => {
                    let val = self.input.pop().unwrap();
                    self.set_val(self.inst_pointer + 1, val, mode_1);
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
                    let (fst, snd) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                    );
                    self.set_val(self.inst_pointer + 3, if fst < snd { 1 } else { 0 }, mode_3);
                    self.inst_pointer += 4;
                }
                8 => {
                    let (fst, snd) = (
                        self.get_val(self.inst_pointer + 1, mode_1),
                        self.get_val(self.inst_pointer + 2, mode_2),
                    );
                    self.set_val(
                        self.inst_pointer + 3,
                        if fst == snd { 1 } else { 0 },
                        mode_3,
                    );
                    self.inst_pointer += 4;
                }
                9 => {
                    self.relative_base += self.get_val(self.inst_pointer + 1, mode_1);
                    self.inst_pointer += 2;
                }
                99 => return ResultCode::Terminated,
                opcode => panic!(
                    "Opcode must be 1, 2, 3, 4, 5, 6, 7, 8, 9 or 99, receive {} at {}",
                    opcode, self.inst_pointer,
                ),
            }
        }
        unreachable!()
    }

    fn run_program(&mut self, input: i64) -> Vec<i64> {
        let mut input = input;
        while let ResultCode::Output(i) = self.run_one_turn(input) {
            input = i;
        }
        self.input.to_owned()
    }
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect())
}

fn part1(input: &[i64]) -> Vec<i64> {
    IntCodeComputer::new(input).run_program(1)
}

fn part2(input: &[i64]) -> Vec<i64> {
    IntCodeComputer::new(input).run_program(2)
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {:?}", part1(&input));
    println!("part 2: {:?}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_test1() {
        let code = vec![104, 1_125_899_906_842_624, 99];
        let res = part1(&code);
        assert_eq!(res, [1, 1_125_899_906_842_624]);
    }

    #[test]
    fn day9_test2() {
        let code = vec![
            9, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let res = part1(&code);
        assert_eq!(
            res,
            [1, 9, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn day9_test3() {
        let code = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let res = part1(&code);
        assert_eq!(res, [1, 1_219_070_632_396_864]);
    }
}
