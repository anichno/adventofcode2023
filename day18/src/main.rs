fn parse(input: &[&str]) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    points.push((0, 0));
    let mut cur_x = 0;
    let mut cur_y = 0;
    for line in input {
        let (dir, line) = line.split_once(' ').unwrap();
        let (dist, _) = line.split_once(' ').unwrap();
        let dist: i64 = dist.parse().unwrap();
        match dir {
            "R" | "L" => {
                let new_x = if dir == "R" {
                    cur_x + dist
                } else {
                    cur_x - dist
                };
                cur_x = new_x;
            }
            "U" | "D" => {
                let new_y = if dir == "D" {
                    cur_y + dist
                } else {
                    cur_y - dist
                };
                cur_y = new_y;
            }
            _ => panic!("invalid dir: {dir}"),
        }
        points.push((cur_x, cur_y));
    }

    points
}

fn solve1(input: &[&str]) -> i64 {
    let points = parse(input);

    let perimeter = (0..points.len() - 1)
        .map(|idx| {
            (points[idx].0 - points[idx + 1].0).abs() + (points[idx].1 - points[idx + 1].1).abs()
        })
        .sum::<i64>();

    (0..points.len() - 1)
        .map(|idx| points[idx].0 * points[idx + 1].1 - points[idx + 1].0 * points[idx].1)
        .sum::<i64>()
        / 2
        + perimeter / 2
        + 1
}

fn parse_part2(input: &[&str]) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    let mut cur_x = 0;
    let mut cur_y = 0;
    points.push((0, 0));
    for line in input {
        let (_, dist) = line.split_once('#').unwrap();
        let dir = dist.as_bytes()[5];
        let dist = &dist[0..5];
        let dist = i64::from_str_radix(dist, 16).unwrap();
        match dir {
            b'0' | b'2' => {
                let new_x = if dir == b'0' {
                    cur_x + dist
                } else {
                    cur_x - dist
                };

                cur_x = new_x;
            }
            b'3' | b'1' => {
                let new_y = if dir == b'1' {
                    cur_y + dist
                } else {
                    cur_y - dist
                };

                cur_y = new_y;
            }
            _ => panic!("invalid dir: {dir}"),
        }
        points.push((cur_x, cur_y))
    }

    points
}

fn solve2(input: &[&str]) -> i64 {
    let points = parse_part2(input);

    let perimeter = (0..points.len() - 1)
        .map(|idx| {
            (points[idx].0 - points[idx + 1].0).abs() + (points[idx].1 - points[idx + 1].1).abs()
        })
        .sum::<i64>();

    (0..points.len() - 1)
        .map(|idx| points[idx].0 * points[idx + 1].1 - points[idx + 1].0 * points[idx].1)
        .sum::<i64>()
        / 2
        + perimeter / 2
        + 1
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 62)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 952408144115)
    }
}
