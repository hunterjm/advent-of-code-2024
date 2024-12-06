use std::collections::HashSet;

advent_of_code::solution!(6);

fn init_map(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    // Find the guard position
    let mut guard = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '^' {
                guard = (x as i32, y as i32);
            }
        }
    }

    // Build the map replacing the guard position with an empty space
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '^' { '.' } else { c })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (map, guard)
}

fn run_game(
    map: &[Vec<char>],
    guard: (i32, i32),
    test_pos: Option<(usize, usize)>,
) -> (bool, usize) {
    // Setup the game
    let mut position = guard;
    let mut direction = (0, -1); // Up
    let mut positions = HashSet::new();
    let mut state_history = HashSet::new();

    positions.insert(position);
    state_history.insert((position, direction));

    // Run the game loop
    loop {
        let next_position = (position.0 + direction.0, position.1 + direction.1);

        // Check if we will go out of bounds
        if next_position.0 < 0
            || next_position.0 >= map[0].len() as i32
            || next_position.1 < 0
            || next_position.1 >= map.len() as i32
        {
            return (false, positions.len());
        }

        // Check if we hit an obstacle
        let is_obstacle = if let Some(pos) = test_pos {
            (next_position.1 as usize, next_position.0 as usize) == pos
                || map[next_position.1 as usize][next_position.0 as usize] == '#'
        } else {
            map[next_position.1 as usize][next_position.0 as usize] == '#'
        };

        if is_obstacle {
            // Change direction
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            };
        } else {
            // Move to the next position
            position = next_position;
            positions.insert(position);
        }

        // Save the position and direction to check for loops
        if !state_history.insert((position, direction)) {
            // We are in a loop
            return (true, positions.len());
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, guard) = init_map(input);
    let (_, positions) = run_game(&map, guard, None);
    Some(positions as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, guard) = init_map(input);
    let mut loop_count = 0;

    // Just brute force it
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '.'
                && (y as i32, x as i32) != guard
                && run_game(&map, guard, Some((y, x))).0
            {
                loop_count += 1;
            }
        }
    }

    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
