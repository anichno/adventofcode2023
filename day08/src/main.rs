use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

struct Node {
    left: &'static str,
    right: &'static str,
}

fn parse(input: &[&'static str]) -> (Vec<Dir>, HashMap<&'static str, Node>) {
    let mut dirs = Vec::new();
    for chr in input[0].chars() {
        match chr {
            'L' => dirs.push(Dir::Left),
            'R' => dirs.push(Dir::Right),
            _ => panic!("invalid direction: {chr}"),
        }
    }

    let parens: &[_] = &['(', ')'];
    let mut nodes = HashMap::new();
    for line in input.iter().skip(2) {
        let (node_name, line) = line.split_once(" = ").unwrap();
        let (left, right) = line.trim_matches(parens).split_once(", ").unwrap();
        nodes.insert(node_name, Node { left, right });
    }

    (dirs, nodes)
}

fn solve1(input: &[&'static str]) -> u32 {
    let (dirs, nodes) = parse(input);
    let mut num_steps = 0;

    let mut cur_node = "AAA";
    let mut dirs = dirs.into_iter().cycle();
    while cur_node != "ZZZ" {
        let next_dir = dirs.next().unwrap();
        match next_dir {
            Dir::Left => cur_node = nodes[cur_node].left,
            Dir::Right => cur_node = nodes[cur_node].right,
        }
        num_steps += 1;
    }

    num_steps
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a > 0 && b > 0 {
        (a, b) = (a.max(b), a.min(b));
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn solve2(input: &[&'static str]) -> u64 {
    let (dirs, nodes) = parse(input);
    let start_nodes: HashSet<&str> = nodes.keys().filter(|k| k.ends_with('A')).copied().collect();

    let mut loop_intervals = Vec::new();
    for snode in start_nodes {
        let mut seen_ends = HashMap::new();
        let mut steps: u64 = 0;
        let mut cur_node = snode;
        for dir in dirs.iter().cycle() {
            steps += 1;
            match dir {
                Dir::Left => cur_node = nodes[cur_node].left,
                Dir::Right => cur_node = nodes[cur_node].right,
            }

            if cur_node.ends_with('Z') {
                if seen_ends.contains_key(cur_node) {
                    loop_intervals.push(steps - seen_ends.get(cur_node).unwrap());
                    break;
                } else {
                    seen_ends.insert(cur_node, steps);
                }
            }
        }
    }

    loop_intervals.iter().fold(1, |a, v| lcm(a, *v))
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &[&str] = &[
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    const INPUT2: &[&str] = &[
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    const INPUT3: &[&str] = &[
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT1), 2);
        assert_eq!(solve1(INPUT2), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT3), 6)
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 27), 9);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(21, 6), 42);
    }
}
