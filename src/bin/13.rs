use std::cmp::min;

advent_of_code::solution!(13);

/// struct to hold Machine info
struct Machine {
    a_x: i64,
    a_y: i64,
    a_cost: i64,
    b_x: i64,
    b_y: i64,
    b_cost: i64,
    target_x: i64,
    target_y: i64,
}

/// Parse input and initialize Machines
fn init(input: &str, offset: i64) -> Vec<Machine> {
    input
        .split("\n\n") // Split on double newlines first
        .filter(|block| !block.is_empty())
        .map(|block| {
            let lines: Vec<&str> = block.lines().collect();

            // Parse Button A
            let parts: Vec<&str> = lines[0].split(", ").collect();
            let ax = parts[0].trim_start_matches("Button A: X+").parse().unwrap();
            let ay = parts[1].trim_start_matches("Y+").parse().unwrap();

            // Parse Button B
            let parts: Vec<&str> = lines[1].split(", ").collect();
            let bx = parts[0].trim_start_matches("Button B: X+").parse().unwrap();
            let by = parts[1].trim_start_matches("Y+").parse().unwrap();

            // Parse Prize coordinates
            let parts: Vec<&str> = lines[2].split(", ").collect();
            let target_x = parts[0]
                .trim_start_matches("Prize: X=")
                .parse::<i64>()
                .unwrap()
                + offset;
            let target_y = parts[1].trim_start_matches("Y=").parse::<i64>().unwrap() + offset;

            Machine {
                a_x: ax,
                a_y: ay,
                a_cost: 3,
                b_x: bx,
                b_y: by,
                b_cost: 1,
                target_x,
                target_y,
            }
        })
        .collect()
}

/// Calculate Greatest Common Divisor
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// Part 1: Solve machine using brute force
fn solve_machine(machine: &Machine) -> Option<i64> {
    let gcd_x = gcd(machine.a_x, machine.b_x);
    let gcd_y = gcd(machine.a_y, machine.b_y);

    // Check if prize positions are reachable
    if machine.target_x % gcd_x != 0 || machine.target_y % gcd_y != 0 {
        return None;
    }

    let mut min_cost = None;

    // Try different combinations up to 100 presses
    for a in 0..=100 {
        for b in 0..=100 {
            let x = a * machine.a_x + b * machine.b_x;
            let y = a * machine.a_y + b * machine.b_y;

            if x == machine.target_x && y == machine.target_y {
                let cost = a * machine.a_cost + b * machine.b_cost;
                min_cost = Some(match min_cost {
                    Some(current_min) => min(current_min, cost),
                    None => cost,
                });
            }
        }
    }

    min_cost
}

/// Part 2: Solve machine using Cramer's rule
fn solve_machine_cramer(machine: &Machine) -> Option<i64> {
    // Calculate determinant
    let det = machine.a_x * machine.b_y - machine.a_y * machine.b_x;
    if det == 0 {
        return None;
    }

    // Calculate numerators for A and B using Cramer's rule
    let num_a = machine.target_x * machine.b_y - machine.target_y * machine.b_x;
    let num_b = machine.a_x * machine.target_y - machine.a_y * machine.target_x;

    // Check if we have integer solutions
    if num_a % det != 0 || num_b % det != 0 {
        return None;
    }

    // Calculate A and B
    let a = num_a / det;
    let b = num_b / det;

    // Check if solution is non-negative
    if a >= 0 && b >= 0 {
        // Verify solution
        let x = a * machine.a_x + b * machine.b_x;
        let y = a * machine.a_y + b * machine.b_y;
        if x == machine.target_x && y == machine.target_y {
            return Some(a * machine.a_cost + b * machine.b_cost);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = init(input, 0);
    let mut total_tokens = 0;

    for machine in machines.iter() {
        if let Some(tokens) = solve_machine(machine) {
            total_tokens += tokens;
        }
    }

    Some(total_tokens)
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = init(input, 10_000_000_000_000);
    let mut total_tokens = 0;

    for machine in machines.iter() {
        if let Some(tokens) = solve_machine_cramer(machine) {
            total_tokens += tokens;
        }
    }

    Some(total_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
