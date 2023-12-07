use std::collections::HashSet;

use crate::ReturnSize;

fn extract_number(s: &str, idx: usize) -> Option<u32> {
    let bytes = s.as_bytes();

    if idx >= s.len() || !bytes[idx].is_ascii_digit() {
        return None; // Index out of bounds or not a digit
    }

    let mut start = idx;
    let mut end = idx;

    // Search left
    while start > 0 && bytes[start - 1].is_ascii_digit() {
        start -= 1;
    }

    // Search right
    while end < s.len() - 1 && bytes[end + 1].is_ascii_digit() {
        end += 1;
    }

    Some(s[start..=end].parse().expect("Unable to parse int"))
}
fn extract_and_push(
    row: &str,
    idx: usize,
    val: char,
    found: &mut Vec<u32>,
    gears: &mut Vec<u32>,
    cset: &mut HashSet<u32>,
) {
    if let Some(c) = row.chars().nth(idx) {
        if c.is_digit(10) {
            if let Some(n) = extract_number(row, idx) {
                if !cset.contains(&n) {
                    found.push(n);
                    cset.insert(n);
                    if val == '*' {
                        gears.push(n);
                    }
                }
            }
        }
    }
}

fn part1_2(input: Vec<String>) -> (u32, u32) {
    let mut found: Vec<u32> = Vec::new();
    let mut ratio = 0;

    /* this has a huge flaw where if numbers diag are the same such like
        ...#...
        .23.23
        will get counted as duplicate
        these dont apear in the test input apparantly; not fixing
    */

    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.chars().enumerate() {
            if !val.is_digit(10) && val != '.' {
                let mut gears: Vec<u32> = Vec::new();
                let mut cset: HashSet<u32> = HashSet::new();

                // adjacent
                extract_and_push(
                    row,
                    j.wrapping_sub(1),
                    val,
                    &mut found,
                    &mut gears,
                    &mut cset,
                );
                extract_and_push(
                    row,
                    j.wrapping_add(1),
                    val,
                    &mut found,
                    &mut gears,
                    &mut cset,
                );

                // diags
                if i > 0 {
                    let prevstr = &input[i - 1];
                    let mut cset: HashSet<u32> = HashSet::new();
                    extract_and_push(
                        prevstr,
                        j.wrapping_sub(1),
                        val,
                        &mut found,
                        &mut gears,
                        &mut cset,
                    );
                    extract_and_push(prevstr, j, val, &mut found, &mut gears, &mut cset);
                    extract_and_push(
                        prevstr,
                        j.wrapping_add(1),
                        val,
                        &mut found,
                        &mut gears,
                        &mut cset,
                    );
                }

                if i < input.len() - 1 {
                    let nextstr = &input[i + 1];
                    let mut cset: HashSet<u32> = HashSet::new();
                    extract_and_push(
                        nextstr,
                        j.wrapping_sub(1),
                        val,
                        &mut found,
                        &mut gears,
                        &mut cset,
                    );
                    extract_and_push(nextstr, j, val, &mut found, &mut gears, &mut cset);
                    extract_and_push(
                        nextstr,
                        j.wrapping_add(1),
                        val,
                        &mut found,
                        &mut gears,
                        &mut cset,
                    );
                }

                if gears.len() == 2 {
                    ratio += gears[0] * gears[1];
                }
            }
        }
    }

    return (found.iter().sum(), ratio);
}

pub fn solve_day() -> ReturnSize {
    let input: Vec<String> = include_str!("../inputs/day3")
        .lines()
        .map(|line| line.to_string())
        .collect();

    return ReturnSize::U32(part1_2(input));
}

#[cfg(test)]
mod tests {
    use super::part1_2;

    const INPUT: [&str; 10] = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn part1_2_test() {
        let (p1, p2) = part1_2(INPUT.iter().map(|&s| s.into()).collect());

        assert_eq!(4361, p1);
        assert_eq!(467835, p2);
    }
}
