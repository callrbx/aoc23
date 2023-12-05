use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn parse_map(section: &str) -> HashMap<RangeInclusive<usize>, usize> {
    section
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let dst: usize = parts.next()?.parse().ok()?;
            let src: usize = parts.next()?.parse().ok()?;
            let n: usize = parts.next()?.parse().ok()?;
            Some((src..=src + n, dst))
        })
        .collect()
}

fn trace_seed(seed: usize, maps: &[HashMap<RangeInclusive<usize>, usize>]) -> usize {
    let mut current_value = seed;
    for map in maps {
        if let Some((range, &start_dst)) = map
            .iter()
            .find(|&(range, _)| range.contains(&current_value))
        {
            let offset = current_value - *range.start();
            current_value = start_dst + offset;
        }
    }
    current_value
}

fn part1_2(input: &str) -> (u32, u32) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<usize> = sections[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    // part 2 seeds ranges
    let mut seeds_range: Vec<usize> = (seeds[0]..=(seeds[0] + seeds[1] - 1)).collect();
    seeds_range.extend(seeds[2]..=(seeds[2] + seeds[3] - 1));

    let maps = [
        parse_map(sections[1]),
        parse_map(sections[2]),
        parse_map(sections[3]),
        parse_map(sections[4]),
        parse_map(sections[5]),
        parse_map(sections[6]),
        parse_map(sections[7]),
    ];

    let p1 = seeds
        .into_par_iter()
        .map(|seed| trace_seed(seed, &maps))
        .min()
        .unwrap_or(0);

    // could probably do something fancy with combining ranges, but just throw cores at it
    let p2 = seeds_range
        .into_par_iter()
        .map(|seed| trace_seed(seed, &maps))
        .min()
        .unwrap_or(0);

    return (p1 as u32, p2 as u32);
}

pub fn solve_day() -> (u32, u32) {
    let input = include_str!("../inputs/day5");

    return part1_2(&input);
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
