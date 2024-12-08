use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

// Parse input and return antennas by frequency and grid dimensions
fn parse_antennas(input: &str) -> (HashMap<char, Vec<(isize, isize)>>, isize, isize) {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let height = lines.len() as isize;
    let width = lines[0].len() as isize;

    // Find all antennas and record their positions by frequency (character)
    let mut antennas_by_freq: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas_by_freq
                    .entry(c)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    (antennas_by_freq, width, height)
}

// Generate all possible pairs of antennas for a given frequency
fn get_antenna_pairs(antennas: &[(isize, isize)]) -> Vec<((isize, isize), (isize, isize))> {
    let mut pairs = Vec::new();
    // If less than two antennas, they can't form any antinodes
    if antennas.len() < 2 {
        return pairs;
    }

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            pairs.push((antennas[i], antennas[j]));
        }
    }
    pairs
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennas_by_freq, width, height) = parse_antennas(input);

    // A set to hold unique antinode positions
    let mut antinode_positions = HashSet::new();

    // For each frequency group, consider every pair of antennas
    for antennas in antennas_by_freq.values() {
        for ((x1, y1), (x2, y2)) in get_antenna_pairs(antennas) {
            let dx = x2 - x1;
            let dy = y2 - y1;

            // Now check every point in the map to see if it's an antinode
            for yp in 0..height {
                for xp in 0..width {
                    // Check collinearity: (xp - x1)*dy == (yp - y1)*dx
                    let lhs = (xp - x1) * dy;
                    let rhs = (yp - y1) * dx;
                    if lhs != rhs {
                        continue;
                    }

                    let dist_a2 = (xp - x1).pow(2) + (yp - y1).pow(2);
                    let dist_b2 = (xp - x2).pow(2) + (yp - y2).pow(2);

                    if dist_a2 == 0 && dist_b2 == 0 {
                        // Point is exactly at both antennas (which can't happen unless same position).
                        // Not meaningful as an antinode since there's no distance.
                        continue;
                    }

                    // Check the ratio conditions: distA² = 4*distB² or distB² = 4*distA²
                    if (dist_a2 == 4 * dist_b2) || (dist_b2 == 4 * dist_a2) {
                        antinode_positions.insert((xp, yp));
                    }
                }
            }
        }
    }

    Some(antinode_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennas_by_freq, width, height) = parse_antennas(input);

    // A set to hold unique antinode positions
    let mut antinode_positions = HashSet::new();

    // For each frequency, look at all pairs of antennas
    for antennas in antennas_by_freq.values() {
        for ((x1, y1), (x2, y2)) in get_antenna_pairs(antennas) {
            let dx = x2 - x1;
            let dy = y2 - y1;

            // Now check every point in the map to see if it's an antinode
            for yp in 0..height {
                for xp in 0..width {
                    // Check collinearity: (xp - x1)*dy == (yp - y1)*dx
                    let lhs = (xp - x1) * dy;
                    let rhs = (yp - y1) * dx;
                    if lhs == rhs {
                        // No need to check distance ratios this time
                        antinode_positions.insert((xp, yp));
                    }
                }
            }
        }
    }

    Some(antinode_positions.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
