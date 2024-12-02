pub fn solve_step1(input: &str) -> String {
    //println!("input : {}", input);

    let mut left: Vec<i32>;
    let mut right: Vec<i32>;

    (left, right) = get_vecs_from_input(input);
    left.sort();
    right.sort();

    let mut distances: Vec<i32> = Vec::new();
    for i in 0..left.len() {
        distances.push((left[i] - right[i]).abs());
    }
    let mut total = 0;
    for value in distances {
        total += value;
    }
    total.to_string()
}

pub fn solve_step2(input: &str) -> String {
    let left: Vec<i32>;
    let right: Vec<i32>;

    (left, right) = get_vecs_from_input(input);

    let mut occurences : Vec<i32> = Vec::new();
    for value in left {
        occurences.push(value * right.iter().filter(|&x| *x == value).count() as i32);
    }
    let mut total = 0;
    for value in occurences {
        total += value;
    }
    total.to_string()
}

pub fn get_vecs_from_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            left.push(l.parse::<i32>().unwrap());
            right.push(r.parse::<i32>().unwrap());
        }
    }
    (left, right)
}