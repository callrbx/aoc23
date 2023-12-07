use std::cmp::min;

use crate::ReturnSize;

// obvious approach - fast for p1, slow for p2
/*
fn map_number(num: i64, mapping: &Vec<(i64, i64, i64)>) -> i64 {
    for &(dest_start, source_start, length) in mapping {
        if num >= source_start && num < source_start + length {
            return dest_start + (num - source_start);
        }
    }
    num
}

fn trace_seed(seed: i64, mappings: &Vec<Vec<(i64, i64, i64)>>) -> i64 {
    let mut current_value = seed;
    for mapping in mappings {
        current_value = map_number(current_value, mapping);
    }
    current_value
}
*/

fn parse_map(section: &str) -> Vec<(i64, i64, i64)> {
    section
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let dest_start: i64 = parts.next()?.parse().ok()?;
            let source_start: i64 = parts.next()?.parse().ok()?;
            let length: i64 = parts.next()?.parse().ok()?;
            Some((dest_start, source_start, length))
        })
        .collect()
}

fn remap(lo: i64, hi: i64, m: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    let mut ans = Vec::new();

    for &(dst, src, r) in m.iter() {
        let end = src + r - 1;
        let d = dst - src; // range shift distance
        if !(end < lo || src > hi) {
            ans.push((std::cmp::max(src, lo), std::cmp::min(end, hi), d));
        }
    }

    let mut result = Vec::new();

    for i in 0..ans.len() {
        let (l, r, d) = ans[i];
        result.push((l + d, r + d));

        if i < ans.len() - 1 && ans[i + 1].0 > r + 1 {
            result.push((r + 1, ans[i + 1].0 - 1));
        }
    }

    // handle start and end
    if ans.is_empty() {
        result.push((lo, hi));
    } else {
        if ans[0].0 != lo {
            result.push((lo, ans[0].0 - 1));
        }
        if ans.last().unwrap().1 != hi {
            result.push((ans.last().unwrap().1 + 1, hi));
        }
    }

    result
}

fn part1_2(input: &str) -> (i64, i64) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<i64> = sections[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    // convert initial seeds to seed ranges for p2 approach
    let seed_ranges_p1 = vec![
        (seeds[0], (seeds[0])),
        (seeds[1], (seeds[1])),
        (seeds[2], (seeds[2])),
        (seeds[3], (seeds[3])),
    ];

    // part 2 seeds ranges
    let seed_ranges_p2 = vec![
        (seeds[0], (seeds[0] + seeds[1] - 1)),
        (seeds[2], (seeds[3] + seeds[2] - 1)),
    ];

    let maps = vec![
        parse_map(sections[1]),
        parse_map(sections[2]),
        parse_map(sections[3]),
        parse_map(sections[4]),
        parse_map(sections[5]),
        parse_map(sections[6]),
        parse_map(sections[7]),
    ];

    let mut p1 = i64::MAX;
    for seed_range in seed_ranges_p1.iter() {
        let mut cur_ranges = vec![*seed_range];
        let mut new_ranges;

        for m in maps.iter() {
            new_ranges = Vec::new();
            for &(lo, hi) in cur_ranges.iter() {
                new_ranges.extend(remap(lo, hi, m));
            }
            cur_ranges = new_ranges.clone();
        }

        for (lo, _) in cur_ranges {
            p1 = min(p1, lo);
        }
    }

    // part 2 - need signed math for range wrapping reasons
    let mut p2 = i64::MAX;
    for seed_range in seed_ranges_p2.iter() {
        let mut cur_ranges = vec![*seed_range];
        let mut new_ranges;

        for m in maps.iter() {
            new_ranges = Vec::new();
            for &(lo, hi) in cur_ranges.iter() {
                new_ranges.extend(remap(lo, hi, m));
            }
            cur_ranges = new_ranges.clone();
        }

        for (lo, _) in cur_ranges {
            p2 = min(p2, lo);
        }
    }

    return (p1, p2);
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day5");

    return ReturnSize::I64(part1_2(&input));
}

#[cfg(test)]
mod tests {
    use super::part1_2;

    const INPUT: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn part1_2_test() {
        assert_eq!((35, 46), part1_2(&INPUT));
    }
}
