use std::time::{Duration, Instant};

mod day1;
mod day2;
mod day3;

fn main() {
    let mut total_time = Duration::new(0, 0);
    let day_funcs = Vec::from([day1::solve_day, day2::solve_day, day3::solve_day]);

    for (day, solve_fn) in day_funcs.iter().enumerate() {
        let start_time = Instant::now();
        let (p1, p2) = solve_fn();
        let elapsed = Instant::now() - start_time;
        total_time += elapsed;
        println!("Day {:02} Part1: {}", day + 1, p1);
        println!("Day {:02} Part2: {}", day + 1, p2);
        println!("Day {:02} Time : {}us\n", day + 1, elapsed.as_micros());
    }

    // total time
    println!("Total Solve Time: {}us", total_time.as_micros());
}
