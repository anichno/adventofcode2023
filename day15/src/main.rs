fn run_hash(input: &str) -> u32 {
    let mut cur_val = 0;

    for chr in input.bytes() {
        cur_val += chr as u32;
        cur_val *= 17;
        cur_val %= 256;
    }

    cur_val
}

fn solve1(input: &str) -> u32 {
    let mut total = 0;
    for substr in input.trim().split(',') {
        total += run_hash(substr);
    }

    total
}

enum Operation {
    Remove,
    Insert,
}

fn solve2(input: &str) -> u32 {
    let mut boxes = vec![Vec::new(); 256];
    let split_chars = &['-', '='];
    for instruction in input.trim().split(',') {
        let (label, focal_length) = instruction.split_once(split_chars).unwrap();
        let operation = match instruction.as_bytes()[label.len()..][0] {
            b'-' => Operation::Remove,
            b'=' => Operation::Insert,
            _ => panic!("invalid operation"),
        };
        let focal_length = if !focal_length.is_empty() {
            Some(focal_length.parse::<u32>().unwrap())
        } else {
            None
        };
        let box_idx = run_hash(label) as usize;
        match operation {
            Operation::Remove => boxes[box_idx].retain(|(l, _)| *l != label),
            Operation::Insert => {
                let lense_idx = boxes[box_idx]
                    .iter()
                    .enumerate()
                    .find(|(_, (l, _))| *l == label)
                    .map(|o| o.0);
                if let Some(lense_idx) = lense_idx {
                    boxes[box_idx][lense_idx] = (label, focal_length.unwrap());
                } else {
                    boxes[box_idx].push((label, focal_length.unwrap()))
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(lense_idx, (_, focal_length))| {
                    (box_idx + 1) * (1 + lense_idx) * *focal_length as usize
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u32
}

fn main() {
    let input: &str = include_str!("input.txt");

    println!("part 1: {}", solve1(input));
    println!("part 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_run_hash() {
        assert_eq!(run_hash("HASH"), 52);
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 1320)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 145)
    }
}
