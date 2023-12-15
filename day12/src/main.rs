#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    left: Vec<Status>,
    right: Vec<u32>,
}

fn parse(input: &[&str]) -> Vec<Record> {
    let mut records = Vec::new();

    for line in input {
        let (left_str, right_str) = line.split_once(' ').unwrap();
        let left: Vec<Status> = left_str
            .chars()
            .map(|c| match c {
                '.' => Status::Operational,
                '#' => Status::Damaged,
                '?' => Status::Unknown,
                _ => panic!("invalid status: {c}"),
            })
            .collect();

        let right = right_str.split(',').map(|c| c.parse().unwrap()).collect();

        records.push(Record { left, right });
    }

    records
}

fn validate_status(status_line: &[Status], check_line: &[u32]) -> bool {
    let mut cur_broken = false;
    let mut cur_len = 0;
    let mut cur_check = 0;
    for stat in status_line {
        if let Status::Damaged = *stat {
            if !cur_broken {
                cur_broken = true;
                cur_len = 0;
            }

            cur_len += 1;
        } else if cur_broken {
            cur_broken = false;
            if cur_check >= check_line.len() || check_line[cur_check] != cur_len {
                return false;
            }
            cur_check += 1;
        }
    }

    if cur_broken {
        cur_check == check_line.len() - 1 && check_line[cur_check] == cur_len
    } else {
        cur_check == check_line.len()
    }
}

mod part1 {
    use super::*;

    fn get_combos(cur_line: &mut Vec<Status>, record: &Record) -> u64 {
        if cur_line.len() == record.left.len() {
            if validate_status(cur_line, &record.right) {
                1
            } else {
                0
            }
        } else {
            let next_append = record.left[cur_line.len()];

            if let Status::Unknown = next_append {
                let mut tot_valid = 0;
                cur_line.push(Status::Damaged);
                tot_valid += get_combos(cur_line, record);
                cur_line.pop();
                cur_line.push(Status::Operational);
                tot_valid += get_combos(cur_line, record);
                cur_line.pop();

                tot_valid
            } else {
                cur_line.push(next_append);
                let tot_valid = get_combos(cur_line, record);
                cur_line.pop();
                tot_valid
            }
        }
    }

    pub fn solve1(input: &[&str]) -> u64 {
        let records = parse(input);
        let mut total_combos = 0;
        for record in records {
            let mut cur_line = Vec::new();
            total_combos += get_combos(&mut cur_line, &record);
        }

        total_combos
    }
}

mod part2 {
    use std::collections::HashMap;

    use super::*;

    fn get_combos(
        corrupted_record: &[Status],
        record_checksum: &[u32],
        lookup: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        if let Some(known) = lookup.get(&(corrupted_record.len(), record_checksum.len())) {
            return *known;
        }

        if record_checksum.is_empty() {
            if corrupted_record
                .iter()
                .any(|s| matches!(s, Status::Damaged))
            {
                // invalid
                return 0;
            } else {
                return 1;
            }
        }

        let mut total = 0;
        // we can assume that the spring at index 0 MUST be an operational spring, dividing the group
        for i in 1..corrupted_record.len() {
            let cur_spring = corrupted_record[i];

            if matches!(cur_spring, Status::Damaged | Status::Unknown)
                && (corrupted_record
                    .iter()
                    .skip(i)
                    .take_while(|r| !matches!(r, Status::Operational))
                    .count() as u32)
                    >= record_checksum[0]
                && !matches!(corrupted_record[i - 1], Status::Damaged)
                && !matches!(
                    corrupted_record[i + record_checksum[0] as usize],
                    Status::Damaged
                )
            {
                total += get_combos(
                    &corrupted_record[i + record_checksum[0] as usize..],
                    &record_checksum[1..],
                    lookup,
                );
            }

            if matches!(cur_spring, Status::Damaged) {
                break;
            }
        }

        lookup.insert((corrupted_record.len(), record_checksum.len()), total);
        total
    }

    pub fn solve2(input: &[&str]) -> u64 {
        let records = parse(input);
        let mut expanded_records = Vec::new();
        for record in records {
            // unfold
            let mut unfolded_record_status = Vec::new();
            let mut unfolded_record_check = Vec::new();
            for _ in 0..5 {
                unfolded_record_status.extend(record.left.iter());
                unfolded_record_status.push(Status::Unknown);
                unfolded_record_check.extend(record.right.iter());
            }
            unfolded_record_status.pop();
            let record = Record {
                left: unfolded_record_status,
                right: unfolded_record_check,
            };
            expanded_records.push(record);
        }

        let total_combos = expanded_records
            .iter()
            .map(|r| {
                let mut corrupted_record = r.left.clone();
                corrupted_record.insert(0, Status::Operational);
                corrupted_record.push(Status::Operational);
                let mut lookup = HashMap::new();
                get_combos(&corrupted_record, &r.right, &mut lookup)
            })
            .sum();

        total_combos
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", part1::solve1(&input));
    println!("part 2: {}", part2::solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn test_validation() {
        let record = parse(&[".??..??...?##. 1,1,3"]);
        let cur_line = &[
            Status::Operational,
            Status::Operational,
            Status::Damaged,
            Status::Operational,
            Status::Operational,
            Status::Operational,
            Status::Damaged,
            Status::Operational,
            Status::Operational,
            Status::Operational,
            Status::Damaged,
            Status::Damaged,
            Status::Damaged,
            Status::Operational,
        ];

        assert!(validate_status(cur_line, &record[0].right));
    }

    #[test]
    fn test1() {
        assert_eq!(part1::solve1(&INPUT[0..1]), 1);
        assert_eq!(part1::solve1(&INPUT[1..2]), 4);
        assert_eq!(part1::solve1(&INPUT[2..3]), 1);
        assert_eq!(part1::solve1(&INPUT[3..4]), 1);
        assert_eq!(part1::solve1(&INPUT[4..5]), 4);
        assert_eq!(part1::solve1(&INPUT[5..6]), 10);
        assert_eq!(part1::solve1(INPUT), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(part2::solve2(&INPUT[0..1]), 1);
        assert_eq!(part2::solve2(&INPUT[1..2]), 16384);
        assert_eq!(part2::solve2(&INPUT[2..3]), 1);
        assert_eq!(part2::solve2(&INPUT[3..4]), 16);
        assert_eq!(part2::solve2(&INPUT[4..5]), 2500);
        assert_eq!(part2::solve2(&INPUT[5..6]), 506250);
        assert_eq!(part2::solve2(INPUT), 525152)
    }
}
