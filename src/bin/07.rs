advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let answer = parts.next().unwrap().parse::<u64>().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (answer, numbers)
        })
        .collect::<Vec<_>>();

    let mut total = 0u64;

    'outer: for (answer, numbers) in input {
        let n = numbers.len();
        for mask in 0..(1 << (n - 1)) {
            let mut result = numbers[0];

            for i in 1..n {
                if mask & (1 << (i - 1)) != 0 {
                    result += numbers[i];
                } else {
                    result *= numbers[i];
                }
            }

            if result == answer {
                total += answer;
                continue 'outer; // Move to next line once we find any solution
            }
        }
    }

    Some(total)
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn evaluate(numbers: &[u64], ops: &[Op]) -> u64 {
    let mut result = numbers[0];
    for (i, &op) in ops.iter().enumerate() {
        match op {
            Op::Add => result += numbers[i + 1],
            Op::Mul => result *= numbers[i + 1],
            Op::Concat => {
                let right = numbers[i + 1];
                let mut digits = 0;
                let mut temp = right;
                while temp > 0 {
                    digits += 1;
                    temp /= 10;
                }
                result = result * 10_u64.pow(digits) + right;
            }
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let answer = parts.next().unwrap().parse::<u64>().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (answer, numbers)
        })
        .collect::<Vec<_>>();

    let mut total = 0u64;

    'outer: for (answer, numbers) in input {
        let n = numbers.len();
        let ops = [Op::Add, Op::Mul, Op::Concat];

        // Try all possible combinations of operators
        for mask in 0..ops.len().pow((n - 1) as u32) {
            let mut operators = Vec::new();
            let mut temp = mask;

            // Convert mask to operator sequence
            for _ in 0..n - 1 {
                operators.push(ops[temp % ops.len()]);
                temp /= ops.len();
            }

            if evaluate(&numbers, &operators) == answer {
                total += answer;
                continue 'outer;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
