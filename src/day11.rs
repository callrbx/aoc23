use crate::ReturnSize;

fn manhattan_distance(
    a: (usize, usize),
    b: (usize, usize),
    empty_rows: &[bool],
    empty_cols: &[bool],
    expansion_factor: i64,
) -> i64 {
    let mut distance = 0;

    // calc row distance
    let row_distance = (a.0 as i64 - b.0 as i64).abs();
    let row_expansion = if a.0 != b.0 {
        empty_rows[a.0.min(b.0)..=a.0.max(b.0)]
            .iter()
            .filter(|&&empty| empty)
            .count() as i64
    } else {
        0
    };
    distance += row_distance + row_expansion * (expansion_factor - 1);

    // calc column distance
    let col_distance = (a.1 as i64 - b.1 as i64).abs();
    let col_expansion = if a.1 != b.1 {
        empty_cols[a.1.min(b.1)..=a.1.max(b.1)]
            .iter()
            .filter(|&&empty| empty)
            .count() as i64
    } else {
        0
    };
    distance += col_distance + col_expansion * (expansion_factor - 1);

    distance
}

fn solve(input: &str, expansion_factor: i64) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut empty_rows = vec![true; rows];
    let mut empty_cols = vec![true; cols];

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '#' {
                empty_rows[r] = false;
                empty_cols[c] = false;
            }
        }
    }

    let mut galaxies = vec![];
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '#' {
                galaxies.push((r, c));
            }
        }
    }

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total_distance += manhattan_distance(
                galaxies[i],
                galaxies[j],
                &empty_rows,
                &empty_cols,
                expansion_factor,
            );
        }
    }

    return total_distance;
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day11");

    return ReturnSize::I64((solve(&input, 2), solve(&input, 1000000)));
}

#[cfg(test)]
mod tests {
    use super::solve;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn solve_test() {
        assert_eq!(374, solve(&INPUT, 2));
        assert_eq!(1030, solve(&INPUT, 10));
        assert_eq!(8410, solve(&INPUT, 100));
    }
}
