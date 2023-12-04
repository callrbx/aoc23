#[derive(Clone)]
struct Card {
    winning: Vec<u32>,
    showing: Vec<u32>,
    matches: u32,
    value: u32,
}

impl Card {
    pub fn new(line: String) -> Self {
        let (_, rem) = line.split_once(":").expect("Wrong card format");
        let (winning, showing) = rem.split_once("|").expect("Wring number format");

        let mut new_card = Card {
            winning: winning
                .split_ascii_whitespace()
                .into_iter()
                .map(|n| n.parse().expect("parse fail"))
                .collect(),
            showing: showing
                .split_ascii_whitespace()
                .into_iter()
                .map(|n| n.parse().expect("parse fail"))
                .collect(),
            matches: 0,
            value: 0,
        };

        for x in &new_card.showing {
            if new_card.winning.contains(&x) {
                new_card.matches += 1;
                if new_card.value == 0 {
                    new_card.value = 1;
                } else {
                    new_card.value *= 2;
                }
            }
        }

        return new_card;
    }
}

fn part1_2(input: &Vec<String>) -> (u32, u32) {
    let cards: Vec<Card> = input
        .iter()
        .map(|line| Card::new(line.to_string()))
        .collect();

    let matches: Vec<u32> = cards.iter().map(|card| card.matches).collect();
    let mut card_instances = vec![1; matches.len()]; // start with 1 instance of each card

    // for each card, iterate over instances; increase instance of following cards
    for i in 0..matches.len() {
        for _ in 0..card_instances[i] {
            let num_matches = matches.get(i).unwrap_or(&0);
            for j in 1..=*num_matches {
                if let Some(next_card_instances) = card_instances.get_mut(i + j as usize) {
                    *next_card_instances += 1;
                }
            }
        }
    }

    return (
        cards.iter().map(|card| card.value).sum(),
        card_instances.iter().sum(),
    );
}

pub fn solve_day() -> (u32, u32) {
    let input = include_str!("../inputs/day4")
        .lines()
        .map(|line| line.to_string())
        .collect();

    return part1_2(&input);
}

#[cfg(test)]
mod tests {
    use super::part1_2;

    const INPUT: [&str; 6] = [
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[test]
    fn part1_test() {
        let (p1, p2) = part1_2(&INPUT.iter().map(|&s| s.into()).collect());

        assert_eq!(13, p1);
        assert_eq!(30, p2);
    }
}
