use std::collections::{BTreeSet, HashSet};

advent_of_code::solution!(10);

/// Use breadth-first search to explore all possible paths that increment by exactly 1
/// - BTreeSet is used to automatically sort and deduplicate endpoints
/// - HashSet is used to efficiently check for unique paths
fn find_paths(input: &str) -> (u32, u32) {
    // Parse input
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // Part 1: Sum of unique height-9 positions reachable
    let mut total_endpoints = 0;
    // Part 2: Sum of unique valid paths to height 9
    let mut total_complete_paths = 0;

    // Directions for movement: up, right, down, left
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Iterate through grid to find all trailheads (height 0)
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                // For each trailhead, track its unique endpoints and paths
                let mut unique_endpoints = BTreeSet::new();
                let mut unique_paths = HashSet::new();

                // BFS queue: (current_position, path_taken)
                let mut queue = vec![((i, j), vec![(i, j)])];

                // BFS exploration of all possible paths from this trailhead
                while let Some((pos, path)) = queue.pop() {
                    let (x, y) = pos;

                    // When we reach height 9, record endpoint and path
                    if grid[x][y] == 9 {
                        unique_endpoints.insert((x, y));
                        unique_paths.insert(path);
                        continue;
                    }

                    // Try each possible movement direction
                    for (dx, dy) in directions.iter() {
                        let nx = (x as i32 + dx) as usize;
                        let ny = (y as i32 + dy) as usize;

                        // Valid next position must:
                        // 1. Be within grid bounds
                        // 2. Be exactly one height higher than current
                        // 3. Not create a cycle in the path
                        if nx < grid.len()
                            && ny < grid[0].len()
                            && grid[nx][ny] == grid[x][y] + 1
                            && !path.contains(&(nx, ny))
                        {
                            let mut new_path = path.clone();
                            new_path.push((nx, ny));
                            queue.push(((nx, ny), new_path));
                        }
                    }
                }

                // Add this trailhead's results to totals
                total_endpoints += unique_endpoints.len();
                total_complete_paths += unique_paths.len();
            }
        }
    }

    (total_endpoints as u32, total_complete_paths as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_paths(input).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_paths(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
