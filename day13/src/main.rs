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

    fn run_program(&mut self) -> Vec<i64> {
        while let ResultCode::Output(i) = self.run_one_turn() {
            self.add_input(i);
        }
        self.input.to_owned()
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum TileType {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl From<i64> for TileType {
    fn from(val: i64) -> Self {
        match val {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            i => panic!("Tile id could only be 0, 1, 2, 3, or 4, have {}", i),
        }
    }
}

impl Into<i64> for TileType {
    fn into(self) -> i64 {
        self as i64
    }
}

struct Object {
    x: i64,
    y: i64,
    init: bool,
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect())
}

fn part1(input: &[i64]) -> i64 {
    let output = IntCodeComputer::new(input).run_program();
    let mut tile_map: HashMap<(i64, i64), TileType> = HashMap::new();
    for chunk in output.chunks_exact(3) {
        let (x, y, tile_id) = (chunk[0], chunk[1], chunk[2]);
        tile_map.insert((x, y), TileType::from(tile_id));
    }

    tile_map
        .values()
        .filter(|&tile_type| tile_type == &TileType::Block)
        .count() as i64
}

fn get_output_triple(computer: &mut IntCodeComputer) -> Option<(i64, i64, i64)> {
    if let ResultCode::Output(x) = computer.run_one_turn() {
        if let ResultCode::Output(y) = computer.run_one_turn() {
            if let ResultCode::Output(z) = computer.run_one_turn() {
                return Some((x, y, z));
            }
        }
    }
    None
}

fn part2(input: &[i64]) -> i64 {
    let mut program = input.to_vec();
    program[0] = 2;
    let mut computer = IntCodeComputer::new(&program);

    let mut paddle = Object {
        x: -1,
        y: -1,
        init: false,
    };
    let mut ball = Object {
        x: -1,
        y: -1,
        init: false,
    };
    let mut score = 0;

    while let Some((x, y, tile)) = get_output_triple(&mut computer) {
        match (x, y, tile) {
            (-1, 0, cur_score) => score = cur_score,
            (x, y, tile) if tile == TileType::Paddle as i64 => {
                paddle = Object { x, y, init: true };
                if ball.init {
                    computer.add_input((ball.x - paddle.x).signum())
                }
            }
            (x, y, tile) if tile == TileType::Ball as i64 => {
                ball = Object { x, y, init: true };
                if paddle.init {
                    computer.add_input((ball.x - paddle.x).signum())
                }
            }
            _ => {}
        }
    }
    score
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
