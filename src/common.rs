pub fn get_input_file(day: u8) -> String {
    return format!("inputs/day{}", day);
}

pub fn print_results(day: u8, p1: String, p2: String) {
    println!("Day {} Part 1: {}", day, p1);
    println!("Day {} Part 2: {}\n", day, p2);
}
