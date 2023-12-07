use crate::ReturnSize;

fn extract_digits_from_line(line: &str) -> Option<(char, char)> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    if digits.is_empty() {
        None
    } else {
        Some((digits[0], *digits.last().unwrap()))
    }
}

fn part1(input: &Vec<String>) -> u32 {
    let results: Vec<u32> = input
        .iter()
        .map(|line| {
            if let Some((first_digit, last_digit)) = extract_digits_from_line(&line) {
                let x: u32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
                x
            } else {
                0
            }
        })
        .collect();

    return results.iter().sum();
}

fn part2(input: &Vec<String>) -> u32 {
    // Account for weird overlap of last/first chars
    // still matches the first digit
    let mapping = [
        ("one", "on1e"),
        ("two", "tw2o"),
        ("three", "thre3e"),
        ("four", "fou4r"),
        ("five", "fiv5e"),
        ("six", "si6x"),
        ("seven", "seve7n"),
        ("eight", "eigh8t"),
        ("nine", "nin9e"),
    ];

    let parsed_lines: Vec<String> = input
        .iter()
        .map(|line| {
            let mut mod_line = line.clone();
            for (spelled, num) in &mapping {
                mod_line = mod_line.replace(spelled, num);
            }
            mod_line
        })
        .collect();

    return part1(&parsed_lines);
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day1")
        .lines()
        .map(|line| line.to_string())
        .collect();

    return ReturnSize::U32((part1(&input), part2(&input)));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const PART1_INPUT: [&str; 4] = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    const PART2_INPUT: [&str; 8] = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
        "8eightwo",
    ];

    #[test]
    fn part1_test() {
        assert_eq!(142, part1(&PART1_INPUT.iter().map(|&s| s.into()).collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            281 + 82,
            part2(&PART2_INPUT.iter().map(|&s| s.into()).collect())
        );
    }
}
