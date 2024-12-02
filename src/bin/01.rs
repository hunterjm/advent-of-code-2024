use itertools::Itertools;
use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Break input out into two lists
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    // Sort the lists
    list1.sort_unstable();
    list2.sort_unstable();

    // Now pair in sorted order (smallest with smallest, etc)
    let result = list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(a, b)| b.abs_diff(a))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Split input into pairs and separate into list and frequency map
    let mut list1 = Vec::new();
    let mut freq_map = HashMap::new();

    for line in input.lines() {
        let (a, b) = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        list1.push(a);
        *freq_map.entry(b).or_insert(0) += 1;
    }

    // sum the list, multiplied by the frequency of the number in the map, returning 0 if the number is not in the map
    let result = list1.iter().map(|n| n * freq_map.get(n).unwrap_or(&0)).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // The result should be 11
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
