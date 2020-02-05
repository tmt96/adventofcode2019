use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const SIZE: usize = 5;
const MASK: u32 = 0b11111_11111_11011_11111_11111;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    value: u32,
}

impl State {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn from_str(input: &[String]) -> Self {
        let mut value = 0;

        for (x, line) in input.iter().enumerate() {
            for (y, ch) in line.char_indices() {
                if ch == '#' {
                    value |= 1 << (x * SIZE + y);
                }
            }
        }

        Self { value }
    }

    fn is_bug(self, x: usize, y: usize) -> bool {
        self.value & (1 << (x * SIZE + y)) != 0
    }

    fn get_neighbors(
        self,
        x: usize,
        y: usize,
        outer: Option<State>,
        inner: Option<State>,
    ) -> Vec<(State, usize, usize)> {
        let mut result = Vec::new();

        if x > 0 {
            result.push((self, x - 1, y));
        } else if let Some(outer) = outer {
            result.push((outer, 1, 2))
        }

        if y > 0 {
            result.push((self, x, y - 1));
        } else if let Some(outer) = outer {
            result.push((outer, 2, 1))
        }

        if x < SIZE - 1 {
            result.push((self, x + 1, y));
        } else if let Some(outer) = outer {
            result.push((outer, 3, 2))
        }

        if y < SIZE - 1 {
            result.push((self, x, y + 1));
        } else if let Some(outer) = outer {
            result.push((outer, 2, 3))
        }

        if let Some(inner) = inner {
            let inner_neighbor: Vec<_> = match (x, y) {
                (2, 1) => (0..5).map(|i| (inner, i, 0)).collect(),
                (2, 3) => (0..5).map(|i| (inner, i, SIZE - 1)).collect(),
                (1, 2) => (0..5).map(|i| (inner, 0, i)).collect(),
                (3, 2) => (0..5).map(|i| (inner, SIZE - 1, i)).collect(),
                _ => vec![],
            };
            result.extend(inner_neighbor);
        }
        result
    }

    fn neighbor_bug_count(
        self,
        x: usize,
        y: usize,
        outer: Option<State>,
        inner: Option<State>,
    ) -> usize {
        self.get_neighbors(x, y, outer, inner)
            .iter()
            .filter(|(state, x, y)| state.is_bug(*x, *y))
            .count()
    }

    fn next_position_state(
        self,
        x: usize,
        y: usize,
        outer: Option<State>,
        inner: Option<State>,
    ) -> u32 {
        match (
            self.is_bug(x, y),
            self.neighbor_bug_count(x, y, outer, inner),
        ) {
            (true, 1) => 1,
            (true, _) => 0,
            (false, 1) | (false, 2) => 1,
            (false, _) => 0,
        }
    }

    fn next_state(self, outer: Option<State>, inner: Option<State>) -> Self {
        let mut value = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                value |= self.next_position_state(x, y, outer, inner) << (x * SIZE + y)
            }
        }
        if inner.is_some() {
            value &= MASK;
        }
        Self { value }
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
        state = state.next_state(None, None);
    }
    state.value
}

fn part2(state: State) -> u32 {
    let duration = 200;
    let mut states = vec![State::new(); duration + 1];
    states[duration / 2] = state;

    for i in 0..duration {
        let mut new_states = vec![State::new(); duration + 1];
        for j in (duration - i - 1) / 2..=(duration + i + 2) / 2 {
            let inner = if j == 0 { None } else { Some(states[j - 1]) };
            let outer = states.get(j + 1).copied();
            new_states[j] = states[j].next_state(outer, inner);
        }
        states = new_states
    }

    states.iter().map(|state| state.value.count_ones()).sum()
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
    Ok(())
}
