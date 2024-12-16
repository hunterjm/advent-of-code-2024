advent_of_code::solution!(15);

fn parse_map(input: &str) -> (Vec<String>, String) {
    let lines = input.lines().collect::<Vec<_>>();

    // Separate map lines from moves lines
    let mut map = Vec::new();
    let mut idx = 0;
    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() && line.starts_with('#') && line.ends_with('#') {
            map.push((*line).to_string());
        } else {
            idx = i;
            break;
        }
    }

    let moves_str: String = lines[idx..].join("").replace('\n', "").replace('\r', "");
    (map, moves_str)
}

// Find robot position
fn find_robot(warehouse: &[Vec<char>]) -> (usize, usize) {
    for r in 0..warehouse.len() {
        for c in 0..warehouse[0].len() {
            if warehouse[r][c] == '@' {
                return (r, c);
            }
        }
    }
    panic!("Robot not found");
}

// Direction lookup
fn dir(ch: char) -> (isize, isize) {
    match ch {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("Invalid direction"),
    }
}

// Check if we can push a chain of boxes
fn can_push(warehouse: &[Vec<char>], r: usize, c: usize, dr: isize, dc: isize) -> bool {
    let nr = (r as isize + dr) as usize;
    let nc = (c as isize + dc) as usize;

    if nc >= warehouse[0].len() {
        return false;
    }

    match warehouse[nr][nc] {
        '.' => true,
        'O' | '[' | ']' => can_push(warehouse, nr, nc, dr, dc),
        _ => false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, moves_str) = parse_map(input);

    // Create the warehouse
    let mut warehouse: Vec<Vec<char>> = map.into_iter().map(|l| l.chars().collect()).collect();
    let height = warehouse.len();
    let width = warehouse[0].len();

    // Find robot position
    let (mut robot_r, mut robot_c) = find_robot(&warehouse);

    // Actually push the boxes
    fn push_boxes(warehouse: &mut [Vec<char>], r: usize, c: usize, dr: isize, dc: isize) {
        let nr = (r as isize + dr) as usize;
        let nc = (c as isize + dc) as usize;
        if warehouse[nr][nc] == 'O' {
            push_boxes(warehouse, nr, nc, dr, dc);
        }
        warehouse[nr][nc] = 'O';
        warehouse[r][c] = '.';
    }

    // Attempt a move
    fn attempt_move(
        warehouse: &mut [Vec<char>],
        robot_r: &mut usize,
        robot_c: &mut usize,
        dr: isize,
        dc: isize,
    ) {
        let nr = (*robot_r as isize + dr) as usize;
        let nc = (*robot_c as isize + dc) as usize;
        match warehouse[nr][nc] {
            '.' => {
                // Move robot
                warehouse[*robot_r][*robot_c] = '.';
                warehouse[nr][nc] = '@';
                *robot_r = nr;
                *robot_c = nc;
            }
            'O' => {
                // Need to push
                if can_push(warehouse, nr, nc, dr, dc) {
                    push_boxes(warehouse, nr, nc, dr, dc);
                    // Now place robot
                    warehouse[*robot_r][*robot_c] = '.';
                    warehouse[nr][nc] = '@';
                    *robot_r = nr;
                    *robot_c = nc;
                }
                // else do nothing if cannot push
            }
            '#' | '@' => {
                // Move blocked, do nothing
            }
            _ => {}
        }
    }

    // Process all moves
    for ch in moves_str.chars() {
        let (dr, dc) = dir(ch);
        attempt_move(&mut warehouse, &mut robot_r, &mut robot_c, dr, dc);
    }

    // Compute sum of GPS coordinates
    let mut sum = 0;
    for r in 0..height {
        for c in 0..width {
            if warehouse[r][c] == 'O' {
                sum += 100 * (r as i32) + (c as i32);
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, moves_str) = parse_map(input);

    // Create the warehouse
    let mut warehouse: Vec<Vec<char>> = {
        let mut result = Vec::new();
        for line in map {
            let mut new_line = String::new();
            for ch in line.chars() {
                match ch {
                    '#' => new_line.push_str("##"),
                    'O' => new_line.push_str("[]"),
                    '.' => new_line.push_str(".."),
                    '@' => new_line.push_str("@."),
                    _ => new_line.push_str(".."), // fallback
                }
            }
            result.push(new_line.chars().collect());
        }
        result
    };
    let height = warehouse.len();
    let width = warehouse[0].len();

    // Find robot
    let (mut robot_r, mut robot_c) = find_robot(&warehouse);

    fn find_connected_boxes(
        warehouse: &[Vec<char>],
        r: usize,
        c: usize,
        dr: isize,
        dc: isize,
        visited: &mut Vec<Vec<bool>>,
    ) -> Vec<(usize, usize)> {
        let mut result = vec![(r, c)];
        visited[r][c] = true;

        if dc != 0 {
            // Moving horizontally - only check left/right
            let directions = vec![(0, 1), (0, -1)];
            for (check_dr, check_dc) in directions {
                let nr = (r as isize + check_dr) as usize;
                let nc = (c as isize + check_dc) as usize;

                if nc < warehouse[0].len()
                    && !visited[nr][nc]
                    && matches!(warehouse[nr][nc], '[' | ']')
                {
                    let mut connected = find_connected_boxes(warehouse, nr, nc, dr, dc, visited);
                    result.append(&mut connected);
                }
            }
        } else {
            // Moving vertically - need to handle double-width boxes
            // First find the left bracket of the current box pair
            let start_c = if warehouse[r][c] == '[' { c } else { c - 1 };

            // Mark both parts of the current box as visited
            if !visited[r][start_c] {
                visited[r][start_c] = true;
                result.push((r, start_c));
            }
            if !visited[r][start_c + 1] {
                visited[r][start_c + 1] = true;
                result.push((r, start_c + 1));
            }

            // Check the next row in the movement direction
            let nr = (r as isize + dr) as usize;
            if nr < warehouse.len() {
                // Check both positions in the next row that could be touching this box
                for nc in start_c..=start_c + 1 {
                    if !visited[nr][nc] && matches!(warehouse[nr][nc], '[' | ']') {
                        let mut connected =
                            find_connected_boxes(warehouse, nr, nc, dr, dc, visited);
                        result.append(&mut connected);
                    }
                }
            }
        }

        result
    }

    fn attempt_move(
        warehouse: &mut [Vec<char>],
        robot_r: &mut usize,
        robot_c: &mut usize,
        dr: isize,
        dc: isize,
    ) {
        let nr = (*robot_r as isize + dr) as usize;
        let nc = (*robot_c as isize + dc) as usize;
        match warehouse[nr][nc] {
            '.' => {
                warehouse[*robot_r][*robot_c] = '.';
                warehouse[nr][nc] = '@';
                *robot_r = nr;
                *robot_c = nc;
            }
            '[' | ']' => {
                // Find all connected boxes
                let mut visited = vec![vec![false; warehouse[0].len()]; warehouse.len()];
                let connected = find_connected_boxes(warehouse, nr, nc, dr, dc, &mut visited);

                // Check if all boxes can be moved
                let can_move = connected
                    .iter()
                    .all(|(r, c)| can_push(warehouse, *r, *c, dr, dc));

                if can_move {
                    // Store the box configuration
                    let boxes: Vec<(char, usize, usize)> = connected
                        .iter()
                        .map(|(r, c)| (warehouse[*r][*c], *r, *c))
                        .collect();

                    // Clear original positions
                    for (_, r, c) in &boxes {
                        warehouse[*r][*c] = '.';
                    }

                    // Place boxes in new positions
                    for (ch, r, c) in boxes {
                        let new_r = (r as isize + dr) as usize;
                        let new_c = (c as isize + dc) as usize;
                        warehouse[new_r][new_c] = ch;
                    }

                    // Move robot
                    warehouse[*robot_r][*robot_c] = '.';
                    warehouse[nr][nc] = '@';
                    *robot_r = nr;
                    *robot_c = nc;
                }
            }
            '#' | '@' => {}
            _ => {}
        }
    }

    // Process moves
    for ch in moves_str.chars() {
        let (dr, dc) = dir(ch);
        attempt_move(&mut warehouse, &mut robot_r, &mut robot_c, dr, dc);
    }

    // Compute sum of GPS coordinates for all boxes
    let mut sum = 0;
    for r in 0..height {
        for c in 0..width {
            if warehouse[r][c] == '[' {
                sum += 100 * (r as i32) + (c as i32);
            }
        }
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
        let result_small = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_small, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
        let result_small = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result_small, Some(1751));
    }
}
