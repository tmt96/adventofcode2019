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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
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
    input: VecDeque<i64>,
    output: Vec<i64>,
    inst_pointer: usize,
    relative_base: i64,
}

impl IntCodeComputer {
    fn new(program: &[i64]) -> Self {
        Self {
            program: program.to_vec(),
            input: VecDeque::new(),
            output: Vec::new(),
            inst_pointer: 0,
            relative_base: 0,
        }
    }

    fn add_input(&mut self, new_input: i64) {
        self.input.push_front(new_input)
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

    fn get_val(&mut self, pos: usize, mode: Mode) -> i64 {
        let res = *self.program.expandable_get(pos);
        match mode {
            Mode::Immediate => res,
            Mode::Position => *self.program.expandable_get(res as usize),
            Mode::Relative => *self
                .program
                .expandable_get((res + self.relative_base) as usize),
        }
    }

    fn set_val(&mut self, pos: usize, val: i64, mode: Mode) {
        let offset = *self.program.expandable_get(pos);
        let res = match mode {
            Mode::Position => offset,
            Mode::Relative => offset + self.relative_base,
            _ => panic!(),
        };
        self.program.expandable_set(res as usize, val)
    }

    fn run_one_turn(&mut self) -> ResultCode {
        while self.inst_pointer < self.program.len() {
            let (opcode, mode_1, mode_2, mode_3) = self.parse_instruction();
            match opcode {
                1 => {
                    let fst = self.get_val(self.inst_pointer + 1, mode_1);
                    let snd = self.get_val(self.inst_pointer + 2, mode_2);
                    self.set_val(self.inst_pointer + 3, fst + snd, mode_3);
                    self.inst_pointer += 4;
                }
                2 => {
                    let fst = self.get_val(self.inst_pointer + 1, mode_1);
                    let snd = self.get_val(self.inst_pointer + 2, mode_2);
                    self.set_val(self.inst_pointer + 3, fst * snd, mode_3);
                    self.inst_pointer += 4;
                }
                3 => {
                    let val = self.input.pop_back().unwrap();
                    self.set_val(self.inst_pointer + 1, val, mode_1);
                    self.inst_pointer += 2;
                }
                4 => {
                    let output = self.get_val(self.inst_pointer + 1, mode_1);
                    self.inst_pointer += 2;
                    self.output.push(output);
                    return ResultCode::Output(output);
                }
                5 => {
                    let fst = self.get_val(self.inst_pointer + 1, mode_1);
                    let snd = self.get_val(self.inst_pointer + 2, mode_2);
                    if fst != 0 {
                        self.inst_pointer = snd as usize
                    } else {
                        self.inst_pointer += 3
                    }
                }
                6 => {
                    let fst = self.get_val(self.inst_pointer + 1, mode_1);
                    let snd = self.get_val(self.inst_pointer + 2, mode_2);
                    if fst == 0 {
                        self.inst_pointer = snd as usize
                    } else {
                        self.inst_pointer += 3
                    }
                }
                7 => {
                    let fst = self.get_val(self.inst_pointer + 1, mode_1);
                    let snd = self.get_val(self.inst_pointer + 2, mode_2);
                    let val = if fst < snd { 1 } else { 0 };
                    self.set_val(self.inst_pointer + 3, val, mode_3);
                    self.inst_pointer += 4;
                }
                8 => {
                    let fst = self.get_val(self.inst_pointer + 1, mode_1);
                    let snd = self.get_val(self.inst_pointer + 2, mode_2);
                    let val = if fst == snd { 1 } else { 0 };
                    self.set_val(self.inst_pointer + 3, val, mode_3);
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

    fn run_program(&mut self) -> Vec<i64> {
        while let ResultCode::Output(_) = self.run_one_turn() {
            // self.add_input(i)
        }
        self.output.to_owned()
    }

    fn get_output_as_ascii(&self) -> String {
        self.output
            .iter()
            .map(|i| *i as u8)
            .map(char::from)
            .collect()
    }
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect())
}

const PROGRAM_1: &str = "NOT A J
NOT C T
AND D T
OR T J
WALK
";

const PROGRAM_2: &str = "OR A J
AND B J
AND C J
NOT J J
AND D J
OR E T
OR H T
AND T J
RUN
";

fn part1(input: &[i64]) -> i64 {
    let mut computer = IntCodeComputer::new(&input);
    for i in PROGRAM_1.bytes().map(|ch| ch as i64) {
        computer.add_input(i)
    }
    let res = computer.run_program();
    // println!("{}", computer.get_output_as_ascii());
    *res.last().unwrap()
}

fn part2(input: &[i64]) -> i64 {
    let mut computer = IntCodeComputer::new(&input);
    for i in PROGRAM_2.bytes().map(|ch| ch as i64) {
        computer.add_input(i)
    }
    let res = computer.run_program();
    // println!("{}", computer.get_output_as_ascii());
    *res.last().unwrap()
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
