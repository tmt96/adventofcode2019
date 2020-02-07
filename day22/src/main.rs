use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Command {
    NewStack,
    Cut(i128),
    IncrementDeal(i128),
}

impl Command {
    fn perform<T>(&self, deck: &mut Vec<T>)
    where
        T: Default + std::clone::Clone + std::marker::Copy,
    {
        match self {
            Self::NewStack => deck.reverse(),
            Self::Cut(i) => {
                let i = *i;
                let len = deck.len();
                if i >= 0 {
                    deck.rotate_left((i as usize) % len)
                } else {
                    deck.rotate_right((-i as usize) % len)
                }
            }
            Self::IncrementDeal(i) => {
                let mut new_deck = vec![T::default(); deck.len()];
                let mut cur_pos = 0;
                for &ele in deck.iter() {
                    new_deck[cur_pos] = ele;
                    cur_pos += *i as usize;
                    if cur_pos >= deck.len() {
                        cur_pos -= deck.len();
                    }
                }
                *deck = new_deck;
            }
        }
    }

    fn trace_back(&self, len: usize, pos: usize) -> usize {
        match self {
            Self::NewStack => len - pos - 1,
            Self::Cut(i) => {
                let i = if *i >= 0 { *i } else { len as i128 + *i };
                (pos + i as usize) % len
            }
            Self::IncrementDeal(i) => {
                // since old_pos * i = pos (mod len),
                // old_pos = pos * inverse(i) (mod len)
                let result = pos as i128 * multiplicative_inverse(*i, len as i128) % len as i128;
                result as usize
            }
        }
    }
}

fn multiplicative_inverse(a: i128, modulo: i128) -> i128 {
    fn extended_gcd(a: i128, b: i128, x: i128, y: i128, modulo: i128) -> i128 {
        assert!(a > b);
        if b == 0 {
            x
        } else {
            extended_gcd(b, a % b, y, (x - y * (a / b)) % modulo, modulo)
        }
    }

    let inverse = extended_gcd(modulo, a, 0, 1, modulo);
    if inverse >= 0 {
        inverse
    } else {
        modulo + inverse
    }
}

fn process_line(line: &str) -> Command {
    if line.starts_with("deal into new stack") {
        Command::NewStack
    } else {
        let value = line
            .trim()
            .rsplit(' ')
            .next()
            .unwrap()
            .parse::<i128>()
            .unwrap();
        if line.starts_with("cut") {
            Command::Cut(value)
        } else {
            Command::IncrementDeal(value)
        }
    }
}

fn part1(input: &[Command]) -> i32 {
    let deck_size = 10007;
    let mut deck: Vec<_> = (0..deck_size).collect();
    for command in input {
        command.perform(&mut deck);
    }
    deck.iter().position(|&i| i == 2019).unwrap() as i32
}

fn trace_back_whole_shuffle(commands: &[Command], deck_size: usize, pos: usize) -> usize {
    commands
        .iter()
        .rev()
        .fold(pos as usize, |i, command| command.trace_back(deck_size, i))
}

fn part2(input: &[Command]) -> i128 {
    // Notice that all trace back steps are linear
    // so the whole shuffle also switch position in a linear equation
    // i.e orig_pos = final_pos * a + b

    let deck_size = 119_315_717_514_047;
    let mut times: i64 = 101_741_582_076_661;

    let b = trace_back_whole_shuffle(input, deck_size, 0);
    let tmp = trace_back_whole_shuffle(input, deck_size, 1);
    let a = if tmp >= b {
        tmp - b
    } else {
        tmp + deck_size - b
    };

    let (mut a, mut b, deck_size) = (a as i128, b as i128, deck_size as i128);
    let mut pos = 2020;

    // calculate by each power of 2 smaller then times
    // as this is the only way to get reasonable run time
    // Go from O(n) to O(log n)
    while times != 0 {
        if times & 1 != 0 {
            pos = (pos * a + b) % deck_size;
        }

        // if x * a + b = y (mod deck) and y * a + b = z (mod deck)
        // then x * a * a + a * b + b = z (mod deck)
        times >>= 1;
        b = (a * b + b) % deck_size;
        a = (a * a) % deck_size;
    }

    pos
}

fn read_input(filepath: &Path) -> std::io::Result<Vec<Command>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let input: Vec<_> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| process_line(&line))
        .collect();
    Ok(input)
}

fn main() -> std::io::Result<()> {
    let filepath = Path::new("./input/input.txt");
    let input = read_input(filepath)?;
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    Ok(())
}
