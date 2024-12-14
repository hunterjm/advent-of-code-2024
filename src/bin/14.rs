use std::collections::HashMap;

advent_of_code::solution!(14);

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let position: Vec<isize> = parts[0][2..]
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();
            let velocity: Vec<isize> = parts[1][2..]
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();

            Robot {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            }
        })
        .collect()
}

// Function to compute new position with wrapping
fn compute_position(start: isize, velocity: isize, time: isize, bound: isize) -> usize {
    (start + velocity * time).rem_euclid(bound) as usize
}

fn calculate_safety_factor(robots: Vec<Robot>, width: isize, height: isize, time: isize) -> usize {
    let mut quadrants: HashMap<&str, usize> = HashMap::new();
    quadrants.insert("top-left", 0);
    quadrants.insert("top-right", 0);
    quadrants.insert("bottom-left", 0);
    quadrants.insert("bottom-right", 0);

    let mid_x = width / 2;
    let mid_y = height / 2;

    for robot in robots {
        let x_new = compute_position(robot.position.0, robot.velocity.0, time, width);
        let y_new = compute_position(robot.position.1, robot.velocity.1, time, height);

        // Skip robots exactly on the middle lines
        if x_new as isize == mid_x || y_new as isize == mid_y {
            continue;
        }

        let quadrant = match (x_new as isize <= mid_x, y_new as isize <= mid_y) {
            (true, true) => "top-left",
            (false, true) => "top-right",
            (true, false) => "bottom-left",
            (false, false) => "bottom-right",
        };

        *quadrants.get_mut(quadrant).unwrap() += 1;
    }

    // Return the product of all quadrant counts
    quadrants.values().product()
}

fn variance(numbers: Vec<usize>) -> f64 {
    let n = numbers.len() as f64;
    let mean = numbers.iter().sum::<usize>() as f64 / n;
    let variance = numbers
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / n;
    variance
}

fn visualize_positions(robots: &Vec<Robot>, time: isize, width: isize, height: isize) {
    println!("\nMessage at time {}:", time);
    let positions: Vec<(usize, usize)> = robots
        .iter()
        .map(|r| {
            (
                compute_position(r.position.0, r.velocity.0, time, width),
                compute_position(r.position.1, r.velocity.1, time, height),
            )
        })
        .collect();

    // Find bounds of actual message
    let min_x = positions.iter().map(|&(x, _)| x).min().unwrap_or(0);
    let max_x = positions
        .iter()
        .map(|&(x, _)| x)
        .max()
        .unwrap_or(width as usize);
    let min_y = positions.iter().map(|&(_, y)| y).min().unwrap_or(0);
    let max_y = positions
        .iter()
        .map(|&(_, y)| y)
        .max()
        .unwrap_or(height as usize);

    // Print only the region containing the message
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.iter().any(|&(px, py)| px == x && py == y) {
                print!("🤖");
            } else {
                print!("🟩");
            }
        }
        println!();
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_robots(input);
    let safety_factor = calculate_safety_factor(robots, 101, 103, 100);
    Some(safety_factor as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let robots = parse_robots(input);
    let width = 101;
    let height = 103;
    let max_time = width * height;

    let mut best_time = 0;
    let mut min_variance = f64::MAX;

    for t in 0..max_time {
        let positions: Vec<(usize, usize)> = robots
            .iter()
            .map(|r| {
                (
                    compute_position(r.position.0, r.velocity.0, t, width),
                    compute_position(r.position.1, r.velocity.1, t, height),
                )
            })
            .collect();

        let xs: Vec<usize> = positions.iter().map(|&(x, _)| x).collect();
        let ys: Vec<usize> = positions.iter().map(|&(_, y)| y).collect();

        let total_variance = variance(xs) + variance(ys);

        if total_variance < min_variance {
            min_variance = total_variance;
            best_time = t;
        }
    }

    visualize_positions(&robots, best_time, width, height);
    Some(best_time as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let robots = parse_robots(&advent_of_code::template::read_file("examples", DAY));
        let result = calculate_safety_factor(robots, 11, 7, 100);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5253));
    }
}
