use crate::ReturnSize;

use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Pipe {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Start,      // S
    Ground,     // .
}

fn determine_start_pipe_shape(grid: &HashMap<Point, Pipe>, start_point: Point) -> Pipe {
    let mut connections = Vec::new();

    let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dx, dy) in &offsets {
        let neighbor_x = start_point.0 as isize + dx;
        let neighbor_y = start_point.1 as isize + dy;
        let neighbor = Point(neighbor_x as usize, neighbor_y as usize);

        if let Some(&pipe) = grid.get(&neighbor) {
            let is_connected = match pipe {
                Pipe::Horizontal => dy == 0,
                Pipe::Vertical => dx == 0,
                Pipe::NorthEast => (dx == 0 && dy == 1) || (dx == -1 && dy == 0),
                Pipe::NorthWest => (dx == 0 && dy == 1) || (dx == 1 && dy == 0),
                Pipe::SouthWest => (dx == 0 && dy == -1) || (dx == 1 && dy == 0),
                Pipe::SouthEast => (dx == 0 && dy == -1) || (dx == -1 && dy == 0),
                _ => false,
            };

            if is_connected {
                connections.push((dx, dy));
            }
        }
    }

    match connections.as_slice() {
        [(1, 0), (0, -1)] | [(0, -1), (1, 0)] => Pipe::NorthEast,
        [(-1, 0), (0, -1)] | [(0, -1), (-1, 0)] => Pipe::NorthWest,
        [(-1, 0), (0, 1)] | [(0, 1), (-1, 0)] => Pipe::SouthWest,
        [(1, 0), (0, 1)] | [(0, 1), (1, 0)] => Pipe::SouthEast,
        [(1, 0), (-1, 0)] => Pipe::Horizontal,
        [(0, 1), (0, -1)] => Pipe::Vertical,
        _ => panic!("failed to find start shape"),
    }
}

fn parse_input(input: &str) -> (HashMap<Point, Pipe>, Point) {
    let mut grid = HashMap::new();
    let mut start_point = Point(0, 0);

    let lines: Vec<&str> = input.lines().collect();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let point = Point(x, y);
            let pipe = match ch {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::NorthEast,
                'J' => Pipe::NorthWest,
                '7' => Pipe::SouthWest,
                'F' => Pipe::SouthEast,
                'S' => {
                    start_point = point;
                    Pipe::Start
                }
                _ => Pipe::Ground,
            };
            grid.insert(point, pipe);
        }
    }

    let start_pipe = determine_start_pipe_shape(&grid, start_point);
    grid.insert(start_point, start_pipe);

    (grid, start_point)
}

fn get_neighbors(point: Point, pipe: Pipe) -> Vec<Point> {
    let Point(x, y) = point;
    match pipe {
        Pipe::Vertical => vec![Point(x, y + 1), Point(x, y - 1)],
        Pipe::Horizontal => vec![Point(x + 1, y), Point(x - 1, y)],
        Pipe::NorthEast => vec![Point(x, y - 1), Point(x + 1, y)],
        Pipe::NorthWest => vec![Point(x, y - 1), Point(x - 1, y)],
        Pipe::SouthWest => vec![Point(x, y + 1), Point(x - 1, y)],
        Pipe::SouthEast => vec![Point(x, y + 1), Point(x + 1, y)],
        Pipe::Start | Pipe::Ground => vec![],
    }
}

fn find_farthest_point(grid: &HashMap<Point, Pipe>, start_point: Point) -> (usize, HashSet<Point>) {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    let mut loop_pipes: HashSet<Point> = HashSet::new();

    // super important!
    loop_pipes.insert(start_point);

    distances.insert(start_point, 0);
    queue.push_back(start_point);

    while let Some(point) = queue.pop_front() {
        let distance = distances[&point];
        if let Some(&pipe) = grid.get(&point) {
            for neighbor in get_neighbors(point, pipe) {
                if grid.get(&neighbor).is_some() && !distances.contains_key(&neighbor) {
                    loop_pipes.insert(neighbor);
                    distances.insert(neighbor, distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    return (*distances.values().max().unwrap(), loop_pipes);
}

// point in polygon
fn in_loop(map: &HashMap<Point, Pipe>, loop_points: &HashSet<Point>) -> HashSet<Point> {
    let dots = map
        .iter()
        .filter(|(p, &c)| c == Pipe::Ground || !loop_points.contains(p))
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    let rights = HashSet::from([Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast]);
    let lefts = HashSet::from([Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest]);
    let mut inside = HashSet::new();
    for dot in dots {
        let left = loop_points
            .iter()
            .filter(|p| p.0 == dot.0 && p.1 < dot.1 && lefts.contains(&map[p]))
            .count();
        let right = loop_points
            .iter()
            .filter(|p| p.0 == dot.0 && p.1 < dot.1 && rights.contains(&map[p]))
            .count();
        if left.min(right) % 2 == 1 {
            inside.insert(*dot);
        }
    }
    inside
}

fn part1_2(input: &str) -> (usize, usize) {
    let (grid, start_point) = parse_input(input);
    let (far_point, loop_pipes) = find_farthest_point(&grid, start_point);

    let enclosed = in_loop(&grid, &loop_pipes);

    return (far_point, enclosed.len());
}

pub fn solve_day() -> ReturnSize {
    let input = include_str!("../inputs/day10");

    return ReturnSize::USIZE(part1_2(&input));
}

#[cfg(test)]
mod tests {
    use super::part1_2;

    const INPUT1: &str = "7-F7-
-FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part1_2_test() {
        assert_eq!(8, part1_2(&INPUT1).0);
        assert_eq!(4, part1_2(&INPUT2).1);
        assert_eq!(10, part1_2(&INPUT3).1);
    }
}
