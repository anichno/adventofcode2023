fn solve1(input: &[&str]) -> u32 {
    let mut sum = 0;

    for line in input {
        let first = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let num = first * 10 + last;
        sum += num;
    }

    sum
}

fn solve2(input: &[&str]) -> u32 {
    let mut sum = 0;

    let text_numbers = &[
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input {
        let mut min_text_pos = usize::MAX;
        let mut min_text_val = None;
        let mut max_text_pos = usize::MIN;
        let mut max_text_val = None;

        for (i, txt_num) in text_numbers.iter().enumerate() {
            if let Some(pos) = line.find(txt_num) {
                if pos < min_text_pos {
                    min_text_pos = pos;
                    min_text_val = Some(i as u32);
                }
            }

            if let Some(pos) = line.rfind(txt_num) {
                if pos > max_text_pos {
                    max_text_pos = pos;
                    max_text_val = Some(i as u32);
                }
            }
        }

        let mut min_num_pos = usize::MAX;
        let mut min_num_val = None;
        let mut max_num_pos = usize::MIN;
        let mut max_num_val = None;

        for (pos, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if pos < min_num_pos {
                    min_num_pos = pos;
                    min_num_val = Some(c.to_digit(10).unwrap());
                }

                if pos > max_num_pos {
                    max_num_pos = pos;
                    max_num_val = Some(c.to_digit(10).unwrap());
                }
            }
        }

        let left = match (min_text_val, min_num_val) {
            (None, None) => todo!(),
            (None, Some(val)) => val,
            (Some(val), None) => val,
            (Some(tval), Some(nval)) => {
                if min_text_pos < min_num_pos {
                    tval
                } else {
                    nval
                }
            }
        };

        let right = match (max_text_val, max_num_val) {
            (None, None) => left,
            (None, Some(val)) => val,
            (Some(val), None) => val,
            (Some(tval), Some(nval)) => {
                if max_text_pos > max_num_pos {
                    tval
                } else {
                    nval
                }
            }
        };

        sum += left * 10 + right;
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

    const INPUT: &[&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 142)
    }

    #[test]
    fn test2() {
        let part_2_input = &[
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(solve2(part_2_input), 281)
    }
}
