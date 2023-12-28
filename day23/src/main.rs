mod part1 {
    use std::collections::HashMap;

    use utils::{Grid, Point};

    fn parse(input: &[&str]) -> (Point, Point, HashMap<Point, Vec<Point>>) {
        let grid: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
        let mut map = HashMap::new();
        let mut start = None;
        let mut end = None;

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let cur_loc = Point { x, y };
                if y == 0 && grid[y][x] == '.' {
                    start = Some(cur_loc);
                } else if y == grid.height() - 1 && grid[y][x] == '.' {
                    end = Some(cur_loc);
                }
                map.insert(cur_loc, Vec::new());
                let targets = map.get_mut(&cur_loc).unwrap();
                if let '^' | '>' | 'v' | '<' = grid[y][x] {
                    targets.push(match grid[y][x] {
                        '^' => Point {
                            x,
                            y: y.checked_sub(1).unwrap(),
                        },
                        '>' => Point { x: x + 1, y },
                        'v' => Point { x, y: y + 1 },
                        '<' => Point {
                            x: x.checked_sub(1).unwrap(),
                            y,
                        },
                        _ => unreachable!(), // checked by if statement
                    });
                } else {
                    for chr in grid.limited_adjacents(x, y) {
                        let target_loc = Point { x: chr.x, y: chr.y };
                        match chr.val {
                            '.' => targets.push(target_loc),
                            '#' => (),
                            '^' if chr.y < y => targets.push(target_loc),
                            '>' if chr.x > x => targets.push(target_loc),
                            'v' if chr.y > y => targets.push(target_loc),
                            '<' if chr.x < x => targets.push(target_loc),
                            '^' | '>' | 'v' | '<' => (),
                            _ => panic!("invalid tile: {}", chr.val),
                        }
                    }
                }
            }
        }

        (start.unwrap(), end.unwrap(), map)
    }

    fn longest_path(
        map: &HashMap<Point, Vec<Point>>,
        end: Point,
        path: &mut Vec<Point>,
        max_path_len: &mut usize,
    ) {
        let cur_loc = path.last().unwrap();
        if *cur_loc == end {
            *max_path_len = (*max_path_len).max(path.len());
        } else {
            for target_loc in map.get(cur_loc).unwrap() {
                if !path.contains(target_loc) {
                    path.push(*target_loc);
                    longest_path(map, end, path, max_path_len);
                    path.pop();
                }
            }
        }
    }

    pub fn solve1(input: &[&str]) -> usize {
        let (start, end, map) = parse(input);
        let mut max_path_len = 0;
        let mut path = vec![start];
        longest_path(&map, end, &mut path, &mut max_path_len);

        max_path_len - 1
    }
}

mod part2 {
    use std::collections::HashMap;

    use utils::{Grid, Point};

    fn parse(input: &[&str]) -> (Point, Point, HashMap<Point, Vec<Point>>) {
        let grid: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
        let mut map = HashMap::new();
        let mut start = None;
        let mut end = None;

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let cur_loc = Point { x, y };
                if y == 0 && grid[y][x] == '.' {
                    start = Some(cur_loc);
                } else if y == grid.height() - 1 && grid[y][x] == '.' {
                    end = Some(cur_loc);
                }
                map.insert(cur_loc, Vec::new());
                let targets = map.get_mut(&cur_loc).unwrap();

                for chr in grid.limited_adjacents(x, y) {
                    let target_loc = Point { x: chr.x, y: chr.y };
                    match chr.val {
                        '.' | '^' | '>' | 'v' | '<' => targets.push(target_loc),
                        '#' => (),
                        _ => panic!("invalid tile: {}", chr.val),
                    }
                }
            }
        }

        (start.unwrap(), end.unwrap(), map)
    }

    fn simplify_map(
        map: &HashMap<Point, Vec<Point>>,
        start: Point,
        end: Point,
    ) -> HashMap<Point, Vec<(Point, usize)>> {
        let mut small_map = HashMap::new();

        for junction_point in map
            .iter()
            .filter(|(_, v)| v.len() >= 3)
            .map(|(p, _)| *p)
            .chain([start, end])
        {
            // find which junction points this connects to
            for dir_choice in map.get(&junction_point).unwrap() {
                let mut prev_loc = junction_point;
                let mut cur_loc = *dir_choice;
                let mut dist = 1;
                while map.get(&cur_loc).unwrap().len() < 3 && cur_loc != start && cur_loc != end {
                    let tmp = cur_loc;
                    cur_loc = *map
                        .get(&cur_loc)
                        .unwrap()
                        .iter()
                        .find(|p| **p != prev_loc)
                        .unwrap();
                    prev_loc = tmp;
                    dist += 1;
                }

                small_map
                    .entry(junction_point)
                    .or_insert(Vec::new())
                    .push((cur_loc, dist));
            }
        }

        small_map
    }

    fn longest_path(
        map: &HashMap<Point, Vec<(Point, usize)>>,
        end: Point,
        path: &mut Vec<Point>,
        dist: usize,
        max_path_len: &mut usize,
    ) {
        let cur_loc = path.last().unwrap();
        if *cur_loc == end {
            *max_path_len = (*max_path_len).max(dist);
        } else {
            for (target_loc, tgt_dist) in map.get(cur_loc).unwrap() {
                if !path.contains(target_loc) {
                    path.push(*target_loc);
                    longest_path(map, end, path, dist + tgt_dist, max_path_len);
                    path.pop();
                }
            }
        }
    }

    pub fn solve2(input: &[&str]) -> usize {
        let (start, end, map) = parse(input);
        let map = simplify_map(&map, start, end);
        let mut max_path_len = 0;
        let mut path = vec![start];
        longest_path(&map, end, &mut path, 0, &mut max_path_len);

        max_path_len
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
        "#.#####################",
        "#.......#########...###",
        "#######.#########.#.###",
        "###.....#.>.>.###.#.###",
        "###v#####.#v#.###.#.###",
        "###.>...#.#.#.....#...#",
        "###v###.#.#.#########.#",
        "###...#.#.#.......#...#",
        "#####.#.#.#######.#.###",
        "#.....#.#.#.......#...#",
        "#.#####.#.#.#########v#",
        "#.#...#...#...###...>.#",
        "#.#.#v#######v###.###v#",
        "#...#.>.#...>.>.#.###.#",
        "#####v#.#.###v#.#.###.#",
        "#.....#...#...#.#.#...#",
        "#.#########.###.#.#.###",
        "#...###...#...#...#.###",
        "###.###.#.###v#####v###",
        "#...#...#.#.>.>.#.>.###",
        "#.###.###.#.###.#.#v###",
        "#.....###...###...#...#",
        "#####################.#",
    ];

    #[test]
    fn test1() {
        assert_eq!(part1::solve1(INPUT), 94)
    }

    #[test]
    fn test2() {
        assert_eq!(part2::solve2(INPUT), 154)
    }
}
