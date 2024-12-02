pub fn solve_step1(input: &str) -> String {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let tmp : Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        if check_order(&tmp) {
            reports.push(tmp);
        }
    }
    let mut valid_reports = 0;

    for report in reports {
        if is_safe_report(&report) {
            valid_reports += 1;
        }
    }
    
    valid_reports.to_string()
}

pub fn solve_step2(input: &str) -> String {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let tmp : Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        reports.push(tmp);
    }
    let mut valid_reports = 0;

    for report in reports {
        if is_safe_report(&report) && check_order(&report) {
            valid_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut modified_report = report.to_vec();
                modified_report.remove(i);
                println!("{:?}", modified_report);
                if is_safe_report(&modified_report) && check_order(&modified_report) {

                    valid_reports += 1;
                    break;
                }
            }
        }
    }

    valid_reports.to_string()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let max_diff = report.windows(2).map(|v| (v[0] - v[1]).abs()).max();
    let min_diff = report.windows(2).map(|v| (v[0] - v[1]).abs()).min();
    match max_diff {
        Some(max) => {
            match min_diff {
                Some(min) => {
                    if min != 0 && max >= 1 && max <= 3 {
                        return true
                    } else {
                        return false
                    }
                },
                None => {
                    return false
                }
            }
        },
        None => {
            return false
        }
    }
}

fn check_order(vec: &Vec<i32>) -> bool {
    if vec.windows(2).all(|w| w[0] < w[1]) {
        true
    } else if vec.windows(2).all(|w| w[0] > w[1]) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

    #[test]
    fn test_step1() {
        assert_eq!(solve_step1(EXAMPLE_INPUT), "2");
    }

    #[test]
    fn test_step2() {
        assert_eq!(solve_step2(EXAMPLE_INPUT), "4");
    }
}