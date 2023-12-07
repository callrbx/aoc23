use crate::ReturnSize;

fn calculate_ways(time: u128, distance: u128) -> u128 {
    let mut ways = 0;
    for hold_time in 0..time {
        let speed = hold_time;
        let travel_time = time - hold_time;
        let total_distance = speed * travel_time;
        if total_distance > distance {
            ways += 1;
        }
    }
    ways
}

fn part1(input: &str) -> u128 {
    let lines: Vec<&str> = input.trim().lines().collect();

    let times: Vec<u128> = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse time"))
        .collect();

    let distances: Vec<u128> = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse distance"))
        .collect();

    if times.len() != distances.len() {
        panic!("Number of times and distances do not match.");
    }

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| calculate_ways(time, distance))
        .product()
}

fn part2(input: &str) -> u128 {
    let lines: Vec<&str> = input.trim().lines().collect();

    let time: u128 = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse()
        .expect("Failed to parse time");

    let distance: u128 = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .replace(" ", "")
        .parse()
        .expect("Failed to parse distance");

    calculate_ways(time, distance)
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day6");

    return ReturnSize::U128((part1(&input), part2(&input)));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "
    Time:      7  15   30
    Distance:  9  40  200
";

    #[test]
    fn part1_test() {
        assert_eq!(288, part1(&INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(71503, part2(&INPUT));
    }
}
