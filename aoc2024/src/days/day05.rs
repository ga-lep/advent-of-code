use std::collections::{HashMap, HashSet};

pub fn solve_step1(input: &str) -> String {
    let (rules, sequences) = parse(input);
    let mut total_count = 0;

    for sequence in sequences {
        if is_correct_sequence(&rules, &sequence) {
            total_count += sequence[sequence.len() / 2];
        }
    }
    total_count.to_string()
}

pub fn solve_step2(input: &str) -> String {
    let (rules, sequences) = parse(input);
    let mut total_count = 0;

    for sequence in sequences {
        if !is_correct_sequence(&rules, &sequence) {
            let mut sequence_toto = sequence.clone();
            sequence_toto.sort_unstable_by(|a, b| {
               match rules.get(a).map(|valid_b| valid_b.contains(b)) {
                   Some(true) => std::cmp::Ordering::Less,
                   _ => std::cmp::Ordering::Greater,
               }
            });
            if is_correct_sequence(&rules, &sequence_toto) {
                total_count += sequence_toto[sequence_toto.len() / 2];
            }
        }
    }
    total_count.to_string()
}

fn parse(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut sequences: Vec<Vec<u32>> = Vec::new();
    let (prefix, suffix) = input.split_once("\n\n").expect("Invalid input format");

    for line in prefix.lines() {
        let (a, b) = line.trim().split_once('|').expect("Invalid  rule format");
        let a = a.parse::<u32>().expect("Invalid number in rule");
        let b = b.parse::<u32>().expect("Invalid number in rule");
        rules.entry(a).or_default().insert(b);
    }

    sequences.extend(
        suffix
            .lines()
            .map(|line| {
                line.trim()
                    .split(',')
                    .map(|v| v.parse::<u32>().expect("Invalid number in sequences"))
                    .collect()
            })
    );

    (rules, sequences)
}

fn is_correct_sequence(rules: &HashMap<u32, HashSet<u32>>, sequence: &[u32]) -> bool {
    for (idx, &current) in sequence.iter().enumerate().rev() {
        if let Some(matching_rule) = rules.get(&current) {
            if sequence[..idx].iter().any(|&value| matching_rule.contains(&value)) {
                return false;
            }
        } else {
            break;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str =
        "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";

    #[test]
    fn test_step1() {
        assert_eq!(crate::days::day05::solve_step1(SAMPLE), "143");
    }

    #[test]
    fn test_step2() {
        assert_eq!(crate::days::day05::solve_step2(SAMPLE), "123");
    }
}