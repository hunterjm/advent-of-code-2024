use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

// Returns (area, perimeter, corners) for each region
fn solve(input: &str) -> Vec<(u32, u32, u32)> {
    // Parse input into a grid of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Initialize sets to keep track of visited cells and results
    let mut visited = HashSet::new();
    let mut results = Vec::new();

    // Helper function to check if a cell is within bounds and has the target character
    let is_target = |r: i32, c: i32, target: char| -> bool {
        r >= 0
            && r < rows as i32
            && c >= 0
            && c < cols as i32
            && grid[r as usize][c as usize] == target
    };

    // Iterate through the grid
    for r in 0..rows {
        for c in 0..cols {
            // If the cell hasn't been visited yet
            if !visited.contains(&(r, c)) {
                // Initialize region and queue for BFS
                let mut region = HashSet::new();
                let mut queue = VecDeque::new();
                let target = grid[r][c];

                // Standard BFS flood fill
                queue.push_back((r, c));
                region.insert((r, c));
                visited.insert((r, c));

                while let Some((curr_r, curr_c)) = queue.pop_front() {
                    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let new_r = (curr_r as i32 + dr) as usize;
                        let new_c = (curr_c as i32 + dc) as usize;

                        if new_r < rows
                            && new_c < cols
                            && !visited.contains(&(new_r, new_c))
                            && grid[new_r][new_c] == target
                        {
                            queue.push_back((new_r, new_c));
                            region.insert((new_r, new_c));
                            visited.insert((new_r, new_c));
                        }
                    }
                }

                // Initialize area, perimeter, and corners
                let area = region.len() as u32;
                let mut perimeter = 0;
                let mut corners = 0;

                // Iterate through the region
                for &(r, c) in &region {
                    let r = r as i32;
                    let c = c as i32;

                    // Count perimeters by checking adjacent cardinal cells
                    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        if !is_target(r + dr, c + dc, target) {
                            perimeter += 1;
                        }
                    }

                    // Count top left, top right, bottom left, and bottom right outer corners
                    if !is_target(r - 1, c, target) && !is_target(r, c - 1, target) {
                        corners += 1;
                    }
                    if !is_target(r - 1, c, target) && !is_target(r, c + 1, target) {
                        corners += 1;
                    }
                    if !is_target(r + 1, c, target) && !is_target(r, c - 1, target) {
                        corners += 1;
                    }
                    if !is_target(r + 1, c, target) && !is_target(r, c + 1, target) {
                        corners += 1;
                    }

                    // Count top left, top right, bottom left, and bottom right inner corners
                    if is_target(r - 1, c, target)
                        && is_target(r, c - 1, target)
                        && !is_target(r - 1, c - 1, target)
                    {
                        corners += 1;
                    }
                    if is_target(r - 1, c, target)
                        && is_target(r, c + 1, target)
                        && !is_target(r - 1, c + 1, target)
                    {
                        corners += 1;
                    }
                    if is_target(r + 1, c, target)
                        && is_target(r, c - 1, target)
                        && !is_target(r + 1, c - 1, target)
                    {
                        corners += 1;
                    }
                    if is_target(r + 1, c, target)
                        && is_target(r, c + 1, target)
                        && !is_target(r + 1, c + 1, target)
                    {
                        corners += 1;
                    }
                }

                results.push((area, perimeter, corners));
            }
        }
    }

    results
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        solve(input)
            .into_iter()
            .map(|(area, perimeter, _)| area * perimeter)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        solve(input)
            .into_iter()
            .map(|(area, _, corners)| area * corners)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
