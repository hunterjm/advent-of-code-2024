advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // Split input into a matrix
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Define the search word
    let word = "XMAS";
    let word_chars: Vec<char> = word.chars().collect();

    // Define the 8 directions to search for the word
    let directions = [
        (-1, -1), // up left
        (-1, 0),  // up
        (-1, 1),  // up right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // down left
        (1, 0),   // down
        (1, 1),   // down right
    ];

    let mut count = 0;

    // Iterate through the grid
    for i in 0..rows {
        for j in 0..cols {
            // Check if the current cell matches the first character of the word
            if grid[i][j] == word_chars[0] {
                // Iterate through the 8 directions
                for (dx, dy) in directions.iter() {
                    let mut found = true;
                    // Iterate through the word
                    for (k, ch) in word_chars.iter().enumerate() {
                        let x = i as i32 + k as i32 * dx;
                        let y = j as i32 + k as i32 * dy;
                        // Check if the cell is out of bounds or doesn't match the word
                        if x < 0
                            || x >= rows as i32
                            || y < 0
                            || y >= cols as i32
                            || grid[x as usize][y as usize] != *ch
                        {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    // Iterate through the grid
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'A' {
                // Check if we can form a valid X pattern
                if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
                    continue;
                }

                let top_left = grid[i - 1][j - 1];
                let top_right = grid[i - 1][j + 1];
                let bottom_left = grid[i + 1][j - 1];
                let bottom_right = grid[i + 1][j + 1];

                // Check left-right diagonal (top_left with bottom_right)
                let valid_lr = (top_left == 'M' && bottom_right == 'S')
                    || (top_left == 'S' && bottom_right == 'M');

                // Check right-left diagonal (top_right with bottom_left)
                let valid_rl = (top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M');

                if valid_lr && valid_rl {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
