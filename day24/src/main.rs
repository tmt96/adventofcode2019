use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    value: u32,
    height: usize,
    width: usize,
}

impl State {
    fn new(height: usize, width: usize) -> Self {
        Self {
            value: 0,
            height,
            width,
        }
    }

    fn from_str(input: &[String]) -> Self {
        let height = input.len();
        let width = input[0].chars().count();
        let mut value = 0;

        for (x, line) in input.iter().enumerate() {
            for (y, ch) in line.char_indices() {
                if ch == '#' {
                    value |= 1 << (x * width + y);
                }
            }
        }

        Self {
            value,
            height,
            width,
        }
    }

    fn is_bug(&self, x: usize, y: usize) -> bool {
        self.value & (1 << (x * self.width + y)) != 0
    }

    fn neighbor_bug_count(&self, x: usize, y: usize) -> u32 {
        let mut result = 0;
        if x > 0 && self.is_bug(x - 1, y) {
            result += 1;
        }
        if y > 0 && self.is_bug(x, y - 1) {
            result += 1;
        }
        if x < self.height - 1 && self.is_bug(x + 1, y) {
            result += 1;
        }
        if y < self.width - 1 && self.is_bug(x, y + 1) {
            result += 1;
        }

        result
    }

    fn next_position_state(&self, x: usize, y: usize) -> u32 {
        match (self.is_bug(x, y), self.neighbor_bug_count(x, y)) {
            (true, 1) => 1,
            (true, _) => 0,
            (false, 1) | (false, 2) => 1,
            (false, _) => 0,
        }
    }

    fn next_state(&self) -> Self {
        let mut value = 0;
        for x in 0..self.height {
            for y in 0..self.width {
                value |= self.next_position_state(x, y) << (x * self.width + y)
            }
        }
        Self { value, ..*self }
    }
}

fn read_input(filepath: &Path) -> std::io::Result<State> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let input: Vec<_> = reader.lines().filter_map(Result::ok).collect();
    Ok(State::from_str(&input))
}

fn part1(state: State) -> u32 {
    let mut all_states = HashSet::new();
    let mut state = state;
    while all_states.insert(state) {
        state = state.next_state();
    }
    state.value
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(input));
    // println!("part 2: {}", part2(input));
    Ok(())
}
