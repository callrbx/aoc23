use std::{
    io,
    time::{Duration, Instant},
};

use crate::common::print_results;

mod common;
mod day1;
mod day2;

fn main() -> io::Result<()> {
    let mut total_time = Duration::new(0, 0);

    // day1
    let start_time = Instant::now();
    let (d1p1, d1p2) = day1::solve_day()?;
    let elapsed = Instant::now() - start_time;
    total_time += elapsed;
    println!("Day 1 Solve Time: {}us", elapsed.as_micros());
    print_results(1, d1p1, d1p2);

    // day2
    let start_time = Instant::now();
    let (d2p1, d2p2) = day2::solve_day()?;
    let elapsed = Instant::now() - start_time;
    total_time += elapsed;
    println!("Day 2 Solve Time: {}us", elapsed.as_micros());
    print_results(2, d2p1, d2p2);

    // total time
    println!("Total Solve Time: {}us", total_time.as_micros());

    return Ok(());
}
