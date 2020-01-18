const PATTERN: [i32; 4] = [0, 1, 0, -1];
const INPUT: &str = "59767332893712499303507927392492799842280949032647447943708128134759829623432979665638627748828769901459920331809324277257783559980682773005090812015194705678044494427656694450683470894204458322512685463108677297931475224644120088044241514984501801055776621459006306355191173838028818541852472766531691447716699929369254367590657434009446852446382913299030985023252085192396763168288943696868044543275244584834495762182333696287306000879305760028716584659188511036134905935090284404044065551054821920696749822628998776535580685208350672371545812292776910208462128008216282210434666822690603370151291219895209312686939242854295497457769408869210686246";

fn get_result_digit(input: &[i32], pos: usize) -> i32 {
    PATTERN
        .iter()
        .flat_map(|i| vec![i; pos])
        .cycle()
        .skip(1)
        .zip(input)
        .map(|(pat, i)| pat * i)
        .sum()
}

fn iter_phase(input: &[i32]) -> Vec<i32> {
    (1..=input.len())
        .map(|i| get_result_digit(input, i).abs() % 10)
        .collect()
}

fn part1(input: &[i32]) -> String {
    let result = (0..100).fold(input.to_vec(), |output, _| iter_phase(&output));
    result[0..8]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("")
}

/// I cheated for pt 2. Two observations:
/// 1. the value of the ith digit in phase n + 1 only depends on the digits from ith to the end for phase n
/// 2. If i > len(input) / 2 then the ith digit in phase n + 1 is the sum of digits from ith to the end for phase n
fn part2(input: &[i32]) -> String {
    let offset = input[0..7].iter().fold(0, |acc, i| acc * 10 + i) as usize;
    let input: Vec<i32> = input
        .iter()
        .rev()
        .copied()
        .cycle()
        .take(input.len() * 10000 - offset)
        .collect();
    let result = (0..100).fold(input, |output, _| {
        output
            .iter()
            .scan(0, |acc, &x| {
                *acc = (*acc + x) % 10;
                Some(*acc)
            })
            .collect()
    });

    result
        .iter()
        .rev()
        .take(8)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn read_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn main() {
    let input = read_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = "80871224585914546619083218645595";
        let res = part1(&read_input(input));
        assert_eq!(res, "24176176")
    }

    #[test]
    fn part1_test2() {
        let input = "19617804207202209144916044189917";
        let res = part1(&read_input(input));
        assert_eq!(res, "73745418")
    }

    #[test]
    fn part1_test3() {
        let input = "69317163492948606335995924319873";
        let res = part1(&read_input(input));
        assert_eq!(res, "52432133")
    }

    #[test]
    fn part2_test1() {
        let input = "03036732577212944063491565474664";
        let res = part2(&read_input(input));
        assert_eq!(res, "84462026")
    }

    #[test]
    fn part2_test2() {
        let input = "02935109699940807407585447034323";
        let res = part2(&read_input(input));
        assert_eq!(res, "78725270")
    }

    #[test]
    fn part2_test3() {
        let input = "03081770884921959731165446850517";
        let res = part2(&read_input(input));
        assert_eq!(res, "53553731")
    }
}
