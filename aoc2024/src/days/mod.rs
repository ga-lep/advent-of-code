mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub fn get_day_solution(day: u8) -> Option<(fn(&str) -> String, fn(&str) -> String)> {
    match day {
        1 => Some((day01::solve_step1, day01::solve_step2)),
        2 => Some((day02::solve_step1, day02::solve_step2)),
        3 => Some((day03::solve_step1, day03::solve_step2)),
        4 => Some((day04::solve_step1, day04::solve_step2)),
        5 => Some((day05::solve_step1, day05::solve_step2)),
        _ => None,
    }
}