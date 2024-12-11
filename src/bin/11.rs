advent_of_code::solution!(11);

use std::collections::HashMap;

/// Transforms a stone according to rules:
/// - 0 becomes 1
/// - Even number of digits splits into two stones
/// - Otherwise multiply by 2024
fn process_stone(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }

    // Get number of digits using log10
    let digits = (num as f64).log10().floor() as u32 + 1;
    if digits % 2 == 0 {
        let half_digits = digits / 2;
        let divisor = 10_u64.pow(half_digits);
        vec![num / divisor, num % divisor]
    } else {
        vec![num * 2024]
    }
}

/// Recursively counts stones after given iterations
/// Uses memoization to avoid recalculating same values
fn count_stones(num: u64, iterations: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    match (iterations, cache.get(&(num, iterations))) {
        (0, _) => 1,
        (_, Some(&count)) => count,
        (_, None) => {
            let count = process_stone(num)
                .into_iter()
                .map(|n| count_stones(n, iterations - 1, cache))
                .sum();
            cache.insert((num, iterations), count);
            count
        }
    }
}

/// Solves both parts using the same logic with different iteration counts
fn solve(input: &str, iterations: u32) -> u64 {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .map(|n| count_stones(n, iterations, &mut HashMap::new()))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
