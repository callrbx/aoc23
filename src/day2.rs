use std::{
    cmp,
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::common;

const CUBE_CONFIG: (u32, u32, u32) = (12, 13, 14);

struct Game {
    id: u32,
    mins: (u32, u32, u32),
    power: u32,
    valid: bool,
}

impl Game {
    pub fn new(input: String) -> Self {
        let mut ng = Game {
            id: 0,
            mins: (0, 0, 0),
            power: 0,
            valid: true,
        };

        let (id, rounds) = input.split_once(":").expect("Invalid Game");
        // strip past "Game "
        ng.id = id[5..].parse().expect("Invalid Game ID");

        for round in rounds.split(";") {
            let mut cv: (u32, u32, u32) = (0, 0, 0);
            for config in round.split(",") {
                if let Some((amt, col)) = config.trim().split_once(" ") {
                    let num: u32 = amt.parse().expect("Invalid Ball Amount");
                    match col {
                        "red" => cv.0 += num,
                        "green" => cv.1 += num,
                        "blue" => cv.2 += num,
                        _ => {}
                    }
                }
            }

            // recaluculate mins
            ng.mins = (
                cmp::max(ng.mins.0, cv.0),
                cmp::max(ng.mins.1, cv.1),
                cmp::max(ng.mins.2, cv.2),
            );

            // do not revalidate an invalid game
            if ng.valid {
                ng.valid = cv.0 <= CUBE_CONFIG.0 && cv.1 <= CUBE_CONFIG.1 && cv.2 <= CUBE_CONFIG.2;
            }
        }

        ng.power = ng.mins.0 * ng.mins.1 * ng.mins.2;

        return ng;
    }
}

fn part1(input: Vec<String>) -> u32 {
    return input
        .iter()
        .map(|game| Game::new(game.to_string()))
        .filter(|game| game.valid)
        .map(|game| game.id)
        .sum();
}

fn part2(input: Vec<String>) -> u32 {
    return input
        .iter()
        .map(|game| Game::new(game.to_string()))
        .map(|game| game.power)
        .sum();
}

pub fn solve_day() -> io::Result<(String, String)> {
    let input_file_name: String = common::get_input_file(2);

    let input_file = File::open(input_file_name)?;
    let reader: BufReader<_> = BufReader::new(input_file);
    let input = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    let part1_ans = part1(input.clone());
    let part2_ans = part2(input);

    return Ok((part1_ans.to_string(), part2_ans.to_string()));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: [&str; 5] = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn part1_test() {
        let sum = part1(INPUT.iter().map(|&s| s.into()).collect());

        assert_eq!(8, sum);
    }

    #[test]
    fn part2_test() {
        let sum = part2(INPUT.iter().map(|&s| s.into()).collect());

        assert_eq!(2286, sum);
    }
}
