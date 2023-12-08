use std::collections::HashMap;

use crate::ReturnSize;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b // prevent potential overflow
}

fn lcm_of_vec(numbers: Vec<usize>) -> usize {
    numbers
        .iter()
        .fold(1, |lcm_so_far, &number| lcm(lcm_so_far, number))
}

fn generate_map(steps: &str) -> HashMap<&str, (&str, &str)> {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    steps.split('\n').for_each(|line| {
        let parts: Vec<&str> = line.split(" = (").collect();
        let node = parts[0];
        let paths: Vec<&str> = parts[1].trim_end_matches(')').split(", ").collect();
        map.insert(node, (paths[0], paths[1]));
    });

    return map;
}

fn steps_to_node(
    start: &str,
    end: &str,
    directions: &str,
    map: &HashMap<&str, (&str, &str)>,
) -> usize {
    let mut current_node = start;
    let mut steps = 0;

    for direction in directions.chars().cycle() {
        steps += 1;
        current_node = match direction {
            'L' => map[current_node].0,
            'R' => map[current_node].1,
            _ => panic!("Invalid direction"),
        };

        if current_node == end {
            break;
        }
    }

    return steps;
}

fn part1(input: &str) -> usize {
    let (directions, steps) = input.split_once("\n\n").expect("failed to split dirs");

    let map = generate_map(steps);
    return steps_to_node("AAA", "ZZZ", directions, &map);
}

fn steps_to_z(start: &str, directions: &str, map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut current_node = start;
    let mut steps = 0;

    for direction in directions.chars().cycle() {
        steps += 1;
        current_node = match direction {
            'L' => map[current_node].0,
            'R' => map[current_node].1,
            _ => panic!("Invalid direction"),
        };

        if current_node.ends_with('Z') {
            break;
        }
    }

    return steps;
}

fn part2(input: &str) -> usize {
    let (directions, steps) = input.split_once("\n\n").expect("failed to split dirs");

    let map = generate_map(steps);

    let start_nodes: Vec<&str> = map
        .keys()
        .filter(|&node| node.ends_with('A'))
        .cloned()
        .collect();

    let min_steps: Vec<usize> = start_nodes
        .iter()
        .map(|start| steps_to_z(start, directions, &map))
        .collect();

    return lcm_of_vec(min_steps);
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day8");

    return ReturnSize::USIZE((part1(&input), part2(&input)));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_test() {
        assert_eq!(2, part1(&INPUT1));
        assert_eq!(6, part1(&INPUT2));
    }

    #[test]
    fn part2_test() {
        assert_eq!(6, part2(&INPUT3));
    }
}
