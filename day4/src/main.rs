use std::collections::HashMap;
use std::ops::RangeBounds;

fn condition_helper(f: impl FnMut(&i32) -> bool, i: i32) -> bool {
    let mut counter_map: HashMap<i32, i32> = HashMap::new();
    let (mut cur_num, mut cur_count) = (10, 0);
    let mut i = i;
    while i > 0 {
        let new_num = i % 10;
        if new_num > cur_num {
            return false;
        } else if new_num == cur_num {
            cur_count += 1;
        } else {
            counter_map.insert(cur_num, cur_count);
            cur_num = new_num;
            cur_count = 1;
        }
        i /= 10;
    }
    counter_map.insert(cur_num, cur_count);
    counter_map.values().any(f)
}

fn part1(range: impl RangeBounds<i32> + std::iter::Iterator<Item = i32>) -> usize {
    range.filter(|&i| condition_helper(|&x| x > 2, i)).count()
}

fn part2(range: impl RangeBounds<i32> + std::iter::Iterator<Item = i32>) -> usize {
    range.filter(|&i| condition_helper(|&x| x == 2, i)).count()
}

fn main() {
    let (low, high) = (256_310, 732_736);
    println!("part1: {}", part1(low..high));
    println!("part2: {}", part2(low..high));
}
