use regex::Regex;

pub fn solve_step1(input: &str) -> String {
    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    let mut total = 0;

    for cap in re.captures_iter(input) {
        if let (Some(first), Some(second)) = (cap.get(1), cap.get(2)) {
            if let (Ok(num1), Ok(num2)) = (first.as_str().parse::<i32>(), second.as_str().parse::<i32>()) {
                total += num1 * num2;
            }
        }
    }
    total.to_string()
}

pub fn solve_step2(input: &str) -> String {
    let mut total = 0;

    let toggle_re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    let mut is_enabled = true;

    for (start, part) in toggle_re.split(input).enumerate() {
        if start > 0 {
            if let Some(toggle) = toggle_re.find(&input[input.find(part).unwrap() - 8..input.find(part).unwrap()]) {
                if toggle.as_str() == "don't()" {
                    is_enabled = false;
                } else if toggle.as_str() == "do()" {
                    is_enabled = true;
                }
            }
        }
        if is_enabled {
            for mul_cap in mul_re.captures_iter(part) {
                if let (Some(first), Some(second)) = (mul_cap.get(1), mul_cap.get(2)) {
                    if let (Ok(num1), Ok(num2)) = (first.as_str().parse::<i32>(), second.as_str().parse::<i32>()) {
                        total += num1 * num2;
                    }
                }
            }
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_STEP1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_INPUT_STEP2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_step1() {
        assert_eq!(solve_step1(EXAMPLE_INPUT_STEP1), "161");
    }

    #[test]
    fn test_step2() {
        assert_eq!(solve_step2(EXAMPLE_INPUT_STEP2), "48");
    }
}