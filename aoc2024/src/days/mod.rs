mod day01;
mod day02;

pub fn get_day_solution(day: u8) -> Option<(fn(&str) -> String, fn(&str) -> String)> {
    match day {
        1 => Some((day01::solve_step1, day01::solve_step2)),
        2 => Some((day02::solve_step1, day02::solve_step2)),
        _ => None,
    }
}