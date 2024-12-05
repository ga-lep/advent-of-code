pub fn solve_step1(input: &str) -> String {

    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.trim().chars().collect());
    }

    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
    ];

    let word_chars: Vec<char> = "XMAS".chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                if matches_word(&grid, &word_chars, row, col, dx, dy) {
                    total_count += 1;
                }
            }
        }
    }
    total_count.to_string()
}


fn matches_word(
    grid: &[Vec<char>],
    word: &[char],
    start_row: usize,
    start_col: usize,
    dx: i32,
    dy: i32,
) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (i, &ch) in word.iter().enumerate() {
        let new_row = start_row as i32 + i as i32 * dx;
        let new_col = start_col as i32 + i as i32 * dy;

        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false; // Out of bounds
        }

        if grid[new_row as usize][new_col as usize] != ch {
            return false; // Character mismatch
        }
    }

    true
}


pub fn solve_step2(input: &str) -> String {
    let mut total_count = 0;
    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.trim().chars().collect());
    }

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] == 'A' {
                if grid[i - 1][j - 1] == 'M'
                    && grid[i + 1][j + 1] == 'S'
                    && grid[i + 1][j - 1] == 'M'
                    && grid[i - 1][j + 1] == 'S'
                {
                    total_count += 1;
                }
                if grid[i - 1][j - 1] == 'S'
                    && grid[i + 1][j + 1] == 'M'
                    && grid[i + 1][j - 1] == 'M'
                    && grid[i - 1][j + 1] == 'S'
                {
                    total_count += 1;
                }
                if grid[i - 1][j - 1] == 'M'
                    && grid[i + 1][j + 1] == 'S'
                    && grid[i + 1][j - 1] == 'S'
                    && grid[i - 1][j + 1] == 'M'
                {
                    total_count += 1;
                }
                if grid[i - 1][j - 1] == 'S'
                    && grid[i + 1][j + 1] == 'M'
                    && grid[i + 1][j - 1] == 'S'
                    && grid[i - 1][j + 1] == 'M'
                {
                    total_count += 1;
                }
            }
        }
    }
    total_count.to_string()
}


#[cfg(test)]
mod tests {
    const SAMPLE: &str =
        "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

    #[test]
    fn test_step1() {
        assert_eq!(crate::days::day04::solve_step1(SAMPLE), "18");
    }

    #[test]
    fn test_step2() {
        assert_eq!(crate::days::day04::solve_step2(SAMPLE), "9");
    }
}