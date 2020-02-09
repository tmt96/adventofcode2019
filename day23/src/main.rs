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
    Input,
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
    is_halted: bool,
}

impl IntCodeComputer {
    fn new(program: &[i64]) -> Self {
        Self {
            program: program.to_vec(),
            input: VecDeque::new(),
            output: Vec::new(),
            inst_pointer: 0,
            relative_base: 0,
            is_halted: false,
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
        while self.inst_pointer < self.program.len() && !self.is_halted {
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
                    if let Some(val) = self.input.pop_back() {
                        self.set_val(self.inst_pointer + 1, val, mode_1);
                        self.inst_pointer += 2;
                    } else {
                        return ResultCode::Input;
                    }
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
                99 => {
                    self.is_halted = true;
                    return ResultCode::Terminated;
                }
                opcode => panic!(
                    "Opcode must be 1, 2, 3, 4, 5, 6, 7, 8, 9 or 99, receive {} at {}",
                    opcode, self.inst_pointer,
                ),
            }
        }
        unreachable!()
    }

    fn is_idle(&self) -> bool {
        let empty_input = self.input.is_empty() || self.input == vec![-1];
        let (opcode, _, _, _) = self.parse_instruction();
        empty_input && opcode == 3
    }

    fn run_program(&mut self) -> Vec<i64> {
        loop {
            match self.run_one_turn() {
                ResultCode::Input => self.input.push_back(-1),
                ResultCode::Terminated => break,
                _ => {}
            }
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

fn run_all_once(computers: &mut [IntCodeComputer], nat: &mut Vec<(i64, i64)>) {
    let len = computers.len();
    for i in 0..len {
        let computer = &mut computers[i];
        if computer.is_halted {
            continue;
        }

        match computer.run_one_turn() {
            ResultCode::Terminated => break,
            ResultCode::Input => computer.add_input(-1),
            ResultCode::Output(_) => {
                let output = &mut computer.output;
                if output.len() == 3 {
                    let (dest, x, y) = (output[0], output[1], output[2]);
                    *output = vec![];

                    if dest == 255 {
                        nat.push((x, y))
                    } else {
                        let dest_comp = &mut computers[dest as usize];
                        dest_comp.add_input(x);
                        dest_comp.add_input(y);
                    }
                }
            }
        }
    }
}

fn part1(input: &[i64]) -> i64 {
    let mut computers: Vec<_> = (0..50)
        .map(|i| {
            let mut computer = IntCodeComputer::new(&input);
            computer.add_input(i);
            computer
        })
        .collect();
    let mut nat = vec![];

    loop {
        run_all_once(&mut computers, &mut nat);
        if let Some(&(_, y)) = nat.first() {
            return y;
        }
    }
}

fn part2(input: &[i64]) -> i64 {
    let mut computers: Vec<_> = (0..50)
        .map(|i| {
            let mut computer = IntCodeComputer::new(&input);
            computer.add_input(i);
            computer
        })
        .collect();
    let mut nat = vec![];
    let mut last_y = None;

    loop {
        run_all_once(&mut computers, &mut nat);

        if computers
            .iter()
            .all(|comp| comp.is_halted || comp.is_idle())
        {
            if let Some(&(x, y)) = nat.last() {
                if last_y == Some(y) {
                    return y;
                }
                computers[0].add_input(x);
                computers[0].add_input(y);
                last_y = Some(y);
            }
        }
    }
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<i64>> {
    Ok(read_to_string(filepath)?
        .split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect())
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
