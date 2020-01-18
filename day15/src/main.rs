use std::collections::{HashSet, VecDeque};
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

    fn run_program(&mut self) -> Vec<i64> {
        while let ResultCode::Output(i) = self.run_one_turn() {
            self.add_input(i);
        }
        self.input.to_owned()
    }
}

type Position = (i64, i64);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum TileType {
    Wall = 0,
    Empty = 1,
    Goal = 2,
}

impl From<i64> for TileType {
    fn from(val: i64) -> Self {
        match val {
            0 => Self::Wall,
            1 => Self::Empty,
            2 => Self::Goal,
            i => panic!("Tile id could only be 0, 1, or 2, have {}", i),
        }
    }
}

impl Into<i64> for TileType {
    fn into(self) -> i64 {
        self as i64
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    North = 1,
    South = 2,
    East = 3,
    West = 4,
}

impl From<i64> for Direction {
    fn from(val: i64) -> Self {
        match val {
            1 => Self::North,
            2 => Self::South,
            3 => Self::East,
            4 => Self::West,
            i => panic!("Tile id could only be 0, 1, or 2, have {}", i),
        }
    }
}

impl Into<i64> for Direction {
    fn into(self) -> i64 {
        self as i64
    }
}

fn get_neighbor((x, y): Position, dir: Direction) -> Position {
    match dir {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
        Direction::East => (x + 1, y),
    }
}

fn trace_path(program: &[i64], path: &[Direction]) -> TileType {
    let mut computer = IntCodeComputer::new(program);

    let result_code = path
        .iter()
        .map(|dir| {
            computer.add_input(*dir as i64);
            computer.run_one_turn()
        })
        .take_while(|&code| code != ResultCode::Terminated)
        .last();

    match result_code {
        Some(ResultCode::Output(i)) => TileType::from(i as i64),
        _ => TileType::Empty,
    }
}

fn find_oxygen(input: &[i64]) -> (Position, Vec<Direction>) {
    let starting_pos = (0, 0);
    let starting_path = vec![];
    let mut all_paths = VecDeque::from(vec![(starting_pos, starting_path.to_vec())]);
    let mut visited = HashSet::new();
    visited.insert(starting_pos);
    while let Some((pos, path)) = all_paths.pop_front() {
        match trace_path(input, &path) {
            TileType::Goal => return (pos, path),
            TileType::Empty => {
                for dir in [
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ]
                .iter()
                {
                    let neighbor = get_neighbor(pos, *dir);
                    let mut new_path = path.to_vec();
                    new_path.push(*dir);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        all_paths.push_back((neighbor, new_path));
                    }
                }
            }
            _ => {}
        }
    }
    unreachable!()
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect())
}

fn part1(input: &[i64]) -> i64 {
    let (_, path) = find_oxygen(input);
    path.len() as i64
}

fn part2(input: &[i64]) -> i64 {
    let (oxygen_pos, oxygen_path) = find_oxygen(input);
    let mut all_paths = VecDeque::from(vec![(oxygen_pos, oxygen_path.to_vec())]);
    let mut visited = HashSet::new();
    visited.insert(oxygen_pos);
    let mut len = oxygen_path.len();

    while let Some((pos, path)) = all_paths.pop_front() {
        match trace_path(input, &path) {
            TileType::Goal | TileType::Empty => {
                for dir in [
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ]
                .iter()
                {
                    let neighbor = get_neighbor(pos, *dir);
                    let mut new_path = path.to_vec();
                    new_path.push(*dir);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        all_paths.push_back((neighbor, new_path));
                    }
                }
                len = path.len();
            }
            _ => {}
        }
    }
    (len - oxygen_path.len()) as i64
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
