use crate::ReturnSize;

fn differences(seq: &Vec<i64>) -> Vec<i64> {
    seq.windows(2).map(|window| window[1] - window[0]).collect()
}

fn differences_prev(seq: &Vec<i64>) -> Vec<i64> {
    seq.windows(2).map(|window| window[0] - window[1]).collect()
}

fn extrapolate_next_value(line: &str) -> i64 {
    let base: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut sequences = vec![base.clone()];

    loop {
        let next = differences(sequences.last().unwrap());
        if next.iter().all(|&x| x == 0) {
            break;
        }
        sequences.push(next);
    }

    return sequences.iter().map(|s| s.last().unwrap()).sum();
}

fn extrapolate_prev_value(line: &str) -> i64 {
    let base: Vec<i64> = line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut sequences = vec![base.clone()];

    loop {
        let prev = differences_prev(sequences.first().unwrap());
        if prev.iter().all(|&x| x == 0) {
            break;
        }
        sequences.insert(0, prev);
    }

    return sequences.iter().map(|s| s.first().unwrap()).sum();
}

fn part1(input: &str) -> i64 {
    input
        .split("\n")
        .map(|line| extrapolate_next_value(line))
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .split("\n")
        .map(|line| extrapolate_prev_value(line))
        .sum()
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day9");

    return ReturnSize::I64((part1(&input), part2(&input)));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_test() {
        assert_eq!(114, part1(&INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(2, part2(&INPUT));
    }
}
