use std::collections::HashMap;
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

    fn add_input(&mut self, new_input: i64) {
        self.input.push(new_input)
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

    fn run_program(&mut self, input: i64) -> Vec<i64> {
        self.add_input(input);
        while let ResultCode::Output(i) = self.run_one_turn() {
            self.add_input(i);
        }
        self.input.to_owned()
    }
}

fn hull_painter(program: &[i64], starting_color: i64) -> HashMap<(i64, i64), i64> {
    assert!(
        starting_color == 0 || starting_color == 1,
        "Color must be either 0 or 1"
    );

    let mut computer = IntCodeComputer::new(program);
    let (mut cur_pos, mut cur_dir) = ((0, 0), (0, 1));
    let mut painted_squares = HashMap::new();
    computer.add_input(starting_color);

    while let ResultCode::Output(color) = computer.run_one_turn() {
        assert!(color == 0 || color == 1, "Color must be either 0 or 1");
        painted_squares.insert(cur_pos, color);

        match computer.run_one_turn() {
            // turn left
            ResultCode::Output(0) => cur_dir = (-cur_dir.1, cur_dir.0),
            // turn right
            ResultCode::Output(1) => cur_dir = (cur_dir.1, -cur_dir.0),
            ResultCode::Output(_) => panic!("Direction must be either 0 or 1"),
            ResultCode::Terminated => break,
        }
        cur_pos = (cur_pos.0 + cur_dir.0, cur_pos.1 + cur_dir.1);

        computer.add_input(*painted_squares.get(&cur_pos).unwrap_or(&0));
    }

    painted_squares
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect())
}

fn part1(input: &[i64]) -> i64 {
    hull_painter(input, 0).len() as i64
}

fn part2(input: &[i64]) -> String {
    let painted_squares = hull_painter(input, 1);

    let (&max_x, &min_x, &max_y, &min_y) = (
        painted_squares.keys().map(|(x, _)| x).max().unwrap(),
        painted_squares.keys().map(|(x, _)| x).min().unwrap(),
        painted_squares.keys().map(|(_, y)| y).max().unwrap(),
        painted_squares.keys().map(|(_, y)| y).min().unwrap(),
    );

    (min_y..=max_y)
        .rev()
        .map(|y| {
            (min_x..=max_x)
                .map(|x| match *painted_squares.get(&(x, y)).unwrap_or(&0) {
                    0 => ' ',
                    1 => '#',
                    _ => panic!("Color should be eitehr 0 or 1"),
                })
                .collect::<String>()
        })
        .fold("".to_string(), |acc, s| format!("{}\n{}", acc, s))
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
