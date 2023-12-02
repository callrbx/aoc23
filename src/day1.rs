

fn extract_digits_from_line(line: &str) -> Option<(char, char)> {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    if digits.is_empty() {
        None
    } else {
        Some((digits[0], *digits.last().unwrap()))
    }
}

fn part1(input: &Vec<String>) -> (u32, Vec<u32>) {
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

    return (results.iter().sum(), results);
}

fn part2(input: &Vec<String>) -> (u32, Vec<u32>) {
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

pub fn solve_day() -> (u32, u32) {
    let input = include_str!("../inputs/day1")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let (part1_ans, _) = part1(&input);
    let (part2_ans, _) = part2(&input);

    return (part1_ans, part2_ans);
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
        let (sum, results) = part1(&PART1_INPUT.iter().map(|&s| s.into()).collect());

        let part1_nums: Vec<u32> = Vec::from([12, 38, 15, 77]);
        assert_eq!(part1_nums, results);
        assert_eq!(142, sum);
    }

    #[test]
    fn part2_test() {
        let (sum, results) = part2(&PART2_INPUT.iter().map(|&s| s.into()).collect());

        let part2_nums: Vec<u32> = Vec::from([29, 83, 13, 24, 42, 14, 76, 82]);
        assert_eq!(part2_nums, results);
        assert_eq!(281 + 82, sum);
    }
}
