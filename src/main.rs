use std::time::{Duration, Instant};
use structopt::{self, StructOpt};

pub enum ReturnSize {
    U32((u32, u32)),
    I64((i64, i64)),
    U128((u128, u128)),
    Str((String, String)),
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[derive(Debug, StructOpt)]
#[structopt(name = "AoC 2023", about = "Solver for Advent of Code 2023 Challenges")]
struct Opt {
    #[structopt(short, long)]
    day: Option<usize>,
}

fn main() {
    let mut total_time = Duration::new(0, 0);
    let day_funcs = Vec::from([
        day1::solve_day,
        day2::solve_day,
        day3::solve_day,
        day4::solve_day,
        day5::solve_day,
        day6::solve_day,
    ]);

    let opt = Opt::from_args();

    let (days_to_run, mut day) = match opt.day {
        Some(day) => (vec![day_funcs[day - 1]], day),
        None => (day_funcs, 1),
    };

    for solve_fn in days_to_run.iter() {
        let start_time = Instant::now();
        let answer = solve_fn();
        let elapsed: Duration = Instant::now() - start_time;

        match answer {
            ReturnSize::U32(ans) => {
                println!("Day {:02} Part 1: {}", day, ans.0);
                println!("Day {:02} Part 2: {}", day, ans.1);
            }
            ReturnSize::I64(ans) => {
                println!("Day {:02} Part 1: {}", day, ans.0);
                println!("Day {:02} Part 2: {}", day, ans.1);
            }
            ReturnSize::U128(ans) => {
                println!("Day {:02} Part 1: {}", day, ans.0);
                println!("Day {:02} Part 2: {}", day, ans.1);
            }
            ReturnSize::Str(ans) => {
                println!("Day {:02} Part 1: {}", day, ans.0);
                println!("Day {:02} Part 2: {}", day, ans.1);
            }
        }
        total_time += elapsed;

        println!("Day {:02} Time D: {}us\n", day, elapsed.as_micros());
        day += 1;
    }

    // total time
    println!("Total Solve Time: {}s", total_time.as_secs_f64());
    println!(
        "Average Solve Time: {}s",
        total_time.as_secs_f64() / days_to_run.len() as f64
    );
}
