use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

advent_of_code::solution!(16);

#[derive(Eq, PartialEq, Copy, Clone)]
struct State {
    cost: u32,
    row: usize,
    col: usize,
    dir: u8, // 0=N,1=E,2=S,3=W
}

// Implement ordering for the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering so that the smallest cost is popped first
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const TURN_COST: u32 = 1000;

struct Maze {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    rows: usize,
    cols: usize,
}

impl Maze {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let rows = grid.len();
        let cols = grid[0].len();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for r in 0..rows {
            for c in 0..cols {
                match grid[r][c] {
                    'S' => start = (r, c),
                    'E' => end = (r, c),
                    _ => continue,
                }
            }
        }

        Self {
            grid,
            start,
            end,
            rows,
            cols,
        }
    }
}

fn run_maze(maze: &Maze) -> (u32, Vec<Vec<Vec<u32>>>) {
    // Directions: 0=N,1=E,2=S,3=W
    let dr = [-1, 0, 1, 0];
    let dc = [0, 1, 0, -1];

    let mut dist = vec![vec![vec![u32::MAX; 4]; maze.cols]; maze.rows];
    dist[maze.start.0][maze.start.1][1] = 0;

    let mut pq = BinaryHeap::new();
    pq.push(State {
        cost: 0,
        row: maze.start.0,
        col: maze.start.1,
        dir: 1,
    });

    let mut best_score = u32::MAX;

    while let Some(State {
        cost,
        row,
        col,
        dir,
    }) = pq.pop()
    {
        if (row, col) == maze.end {
            best_score = cost;
            break;
        }

        // Skip if not best known cost
        if cost > dist[row][col][dir as usize] {
            continue;
        }

        // Move forward
        let nr = (row as isize + dr[dir as usize]) as usize;
        let nc = (col as isize + dc[dir as usize]) as usize;
        if nr < maze.rows && nc < maze.cols && maze.grid[nr][nc] != '#' {
            let new_cost = cost + 1;
            if new_cost < dist[nr][nc][dir as usize] {
                dist[nr][nc][dir as usize] = new_cost;
                pq.push(State {
                    cost: new_cost,
                    row: nr,
                    col: nc,
                    dir,
                });
            }
        }

        // Turn left/right
        for &new_dir in &[(dir + 3) % 4, (dir + 1) % 4] {
            let new_cost = cost + TURN_COST;
            if new_cost < dist[row][col][new_dir as usize] {
                dist[row][col][new_dir as usize] = new_cost;
                pq.push(State {
                    cost: new_cost,
                    row,
                    col,
                    dir: new_dir,
                });
            }
        }
    }

    (best_score, dist)
}

fn reconstruct_best_paths(
    maze: &Maze,
    dist: &Vec<Vec<Vec<u32>>>,
    best_score: u32,
) -> Vec<Vec<bool>> {
    // If unreachable, return all false
    if best_score == u32::MAX {
        return vec![vec![false; maze.cols]; maze.rows];
    }

    // Directions: 0=N,1=E,2=S,3=W
    let dr = [-1, 0, 1, 0];
    let dc = [0, 1, 0, -1];

    // We'll keep track of visited states to avoid infinite loops
    let mut visited = vec![vec![vec![false; 4]; maze.cols]; maze.rows];
    // We'll also keep a boolean grid to mark if a tile is on a best path
    let mut on_best_path = vec![vec![false; maze.cols]; maze.rows];

    // Initialize a queue with all end states that achieve minimal cost
    let mut queue = VecDeque::new();
    for d in 0..4 {
        if dist[maze.end.0][maze.end.1][d] == best_score {
            visited[maze.end.0][maze.end.1][d] = true;
            queue.push_back((maze.end.0, maze.end.1, d));
            on_best_path[maze.end.0][maze.end.1] = true;
        }
    }

    // Perform backward search
    while let Some((r, c, d)) = queue.pop_front() {
        let cost = dist[r][c][d];

        // Check forward predecessor
        let r_prev_f = r.wrapping_sub(dr[d] as usize);
        let c_prev_f = c.wrapping_sub(dc[d] as usize);

        if r_prev_f < maze.rows && c_prev_f < maze.cols && maze.grid[r_prev_f][c_prev_f] != '#' {
            if dist[r_prev_f][c_prev_f][d] != u32::MAX && dist[r_prev_f][c_prev_f][d] + 1 == cost {
                if !visited[r_prev_f][c_prev_f][d] {
                    visited[r_prev_f][c_prev_f][d] = true;
                    on_best_path[r_prev_f][c_prev_f] = true;
                    queue.push_back((r_prev_f, c_prev_f, d));
                } else {
                    on_best_path[r_prev_f][c_prev_f] = true;
                }
            }
        }

        // Check rotation predecessors
        for &new_dir in &[(d + 1) % 4, (d + 3) % 4] {
            if dist[r][c][new_dir] != u32::MAX && dist[r][c][new_dir] + TURN_COST == cost {
                if !visited[r][c][new_dir] {
                    visited[r][c][new_dir] = true;
                    on_best_path[r][c] = true;
                    queue.push_back((r, c, new_dir));
                } else {
                    on_best_path[r][c] = true;
                }
            }
        }
    }

    on_best_path
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    let (best_score, _) = run_maze(&maze);
    Some(best_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    let (best_score, dist) = run_maze(&maze);
    let best_path = reconstruct_best_paths(&maze, &dist, best_score);
    // Return the number of tiles on the best path
    Some(best_path.iter().flatten().filter(|&&b| b).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
        let result_two = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_two, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
        let result_two = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_two, Some(64));
    }
}
