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
}

// We treat all char as u8 for this challenge as Rust's string handling is a pita.
fn gen_map(input: &[i64]) -> Vec<Vec<u8>> {
    let output = IntCodeComputer::new(input).run_program();
    let string = output.iter().map(|i| *i as u8).collect::<Vec<_>>();
    string
        .split(|&ch| char::from(ch) == '\n')
        .map(|segment| segment.to_vec())
        .filter(|segment| !segment.is_empty())
        .collect()
}

// fn convert_to_printable(map: &[Vec<u8>]) -> Vec<String> {
//     map.iter()
//         .map(|line| line.iter().map(|&ch| char::from(ch)).collect())
//         .collect()
// }

fn is_path(row: usize, col: usize, map: &[Vec<u8>]) -> bool {
    let height = map.len();
    let width = map[0].len();
    if row >= height || col >= width {
        false
    } else {
        char::from(map[row][col]) == '#'
    }
}

fn is_intersection(row: usize, col: usize, map: &[Vec<u8>]) -> bool {
    let height = map.len();
    let width = map[0].len();
    if row == 0 || row >= height - 1 || col == 0 || col >= width - 1 {
        false
    } else {
        is_path(row, col, map)
            && is_path(row + 1, col, map)
            && is_path(row - 1, col, map)
            && is_path(row, col + 1, map)
            && is_path(row, col - 1, map)
    }
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect())
}

fn part1(input: &[i64]) -> i64 {
    let map = gen_map(input);
    // for line in convert_to_printable(&map) {
    //     println!("{}", line)
    // }
    let height = map.len();
    let width = map[0].len();

    let mut result = 0;
    for row in 0..height {
        for col in 0..width {
            if is_intersection(row, col, &map) {
                result += row * col
            }
        }
    }
    result as i64
}

// I got a relatively simple map, which made it possible to get the path by eye-balling
fn part2(input: &[i64]) -> i64 {
    let segment_a = "R,4,R,10,R,8,R,4\n";
    let segment_b = "R,10,R,6,R,4\n";
    let segment_c = "R,4,L,12,R,6,L,12\n";
    let path = "A,B,A,B,C,B,C,A,B,C\n";

    let mut program = input.to_vec();
    program[0] = 2;
    let mut computer = IntCodeComputer::new(&program);

    for i in path.bytes().map(|ch| ch as i64) {
        computer.add_input(i);
    }
    for i in segment_a.bytes().map(|ch| ch as i64) {
        computer.add_input(i);
    }
    for i in segment_b.bytes().map(|ch| ch as i64) {
        computer.add_input(i);
    }
    for i in segment_c.bytes().map(|ch| ch as i64) {
        computer.add_input(i);
    }

    // no video
    computer.add_input(b'n' as i64);
    computer.add_input(b'\n' as i64);

    *computer.run_program().last().unwrap()
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
