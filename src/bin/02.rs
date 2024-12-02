advent_of_code::solution!(2);

fn is_safe_sequence(levels: &[u32]) -> bool {
    if levels.len() <= 1 {
        return false;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for pair in levels.windows(2) {
        let diff = pair[1] as i32 - pair[0] as i32;
        let abs_diff = diff.abs();

        if !(1..=3).contains(&abs_diff) {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if is_safe_sequence(&levels) {
                1
            } else {
                0
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            // First check if sequence is already safe
            if is_safe_sequence(&levels) {
                return 1;
            }

            // Try removing each number and check if resulting sequence is safe
            for i in 0..levels.len() {
                let mut test_levels = levels.clone();
                test_levels.remove(i);
                if is_safe_sequence(&test_levels) {
                    return 1;
                }
            }

            0
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
