use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the input for mul(x,y) using regex where x and y are digits.
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Iterate through matches
    let mut result: u32 = 0;
    for (_, [x, y]) in regex.captures_iter(input).map(|c| c.extract()) {
        // Multiply x and y and add to result
        result += x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(?:don't\(\)|do\(\)|mul\((\d+),(\d+)\))").unwrap();

    let mut result: u32 = 0;
    let mut skip = false;

    for cap in regex.captures_iter(input) {
        let full_match = cap.get(0).unwrap().as_str();
        match full_match {
            "don't()" => skip = true,
            "do()" => skip = false,
            _ => {
                // Must be a multiplication match
                if !skip {
                    let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    result += x * y;
                }
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
