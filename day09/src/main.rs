fn parse(input: &[&str]) -> Vec<Vec<i32>> {
    let mut histories = Vec::new();
    for line in input {
        histories.push(
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
    }

    histories
}

fn solve1(input: &[&str]) -> i32 {
    let histories = parse(input);
    let mut sum = 0;

    for history in histories {
        let mut all_diff_zero = false;
        let mut sequence_stack = Vec::new();
        sequence_stack.push(history);
        while !all_diff_zero {
            sequence_stack.push(
                sequence_stack
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|v| v[1] - v[0])
                    .collect(),
            );

            // test if all zero
            all_diff_zero = sequence_stack.last().unwrap().iter().all(|v| *v == 0);
        }

        // find next value
        let next_value = sequence_stack.iter().fold(0, |a, v| a + v.last().unwrap());

        sum += next_value;
    }
    sum
}

fn solve2(input: &[&str]) -> i32 {
    let histories = parse(input);
    let mut sum = 0;

    for history in histories {
        let mut all_diff_zero = false;
        let mut sequence_stack = Vec::new();
        sequence_stack.push(history);
        while !all_diff_zero {
            sequence_stack.push(
                sequence_stack
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|v| v[1] - v[0])
                    .collect(),
            );

            // test if all zero
            all_diff_zero = sequence_stack.last().unwrap().iter().all(|v| *v == 0);
        }

        // find next value
        let next_value = sequence_stack.iter().rev().fold(0, |a, v| v[0] - a);

        sum += next_value;
    }
    sum
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 114)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 2)
    }
}
