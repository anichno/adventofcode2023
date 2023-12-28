use std::collections::{BinaryHeap, HashMap, HashSet};

use utils::{Grid, Point};

fn parse(input: &[&str]) -> (Point, Vec<Vec<bool>>) {
    let mut start = None;
    let mut grid = Vec::new();

    for (y, line) in input.iter().enumerate() {
        let mut row = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            match chr {
                '.' => row.push(false),
                '#' => row.push(true),
                'S' => {
                    start = Some(Point { x, y });
                    row.push(false);
                }
                _ => panic!("invalid tile: {chr}"),
            }
        }
        grid.push(row);
    }

    (start.unwrap(), grid)
}

fn traverse(
    grid: &[Vec<bool>],
    steps_remaining: usize,
    cur_location: Point,
    end_location_map: &mut HashSet<Point>,
    memory: &mut HashSet<(Point, usize)>,
) {
    if steps_remaining == 0 {
        end_location_map.insert(cur_location);
    } else {
        for next_pos in grid.limited_adjacents(cur_location.x, cur_location.y) {
            let next_loc = Point {
                x: next_pos.x,
                y: next_pos.y,
            };
            if !next_pos.val && !memory.contains(&(next_loc, steps_remaining - 1)) {
                // is ground and new
                traverse(
                    grid,
                    steps_remaining - 1,
                    next_loc,
                    end_location_map,
                    memory,
                );
                memory.insert((next_loc, steps_remaining - 1));
            }
        }
    }
}

fn solve1(input: &[&str], num_steps: usize) -> usize {
    let (start, grid) = parse(input);
    let mut end_location_map = HashSet::new();
    let mut memory = HashSet::new();
    traverse(&grid, num_steps, start, &mut end_location_map, &mut memory);

    end_location_map.len()
}

fn dijkstra_distance(graph: &Vec<Vec<bool>>, source: Point) -> HashMap<Point, usize> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct State {
        dist: usize,
        loc: Point,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.dist.cmp(&self.dist)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut distance = HashMap::new();
    let mut next_nodes = BinaryHeap::new();

    for (y, row) in graph.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if !*col {
                distance.insert(Point { x, y }, usize::MAX);
            }
        }
    }
    next_nodes.push(State {
        dist: 0,
        loc: source,
    });

    *distance.get_mut(&source).unwrap() = 0;

    while let Some(state) = next_nodes.pop() {
        let min_node = state.loc;
        assert_ne!(*distance.get(&min_node).unwrap(), usize::MAX);

        if state.dist > *distance.get(&min_node).unwrap() {
            continue;
        }

        for next_loc in graph.limited_adjacents(min_node.x, min_node.y) {
            if !next_loc.val {
                let next_loc = Point {
                    x: next_loc.x,
                    y: next_loc.y,
                };
                let alt = state.dist + 1;
                if alt < *distance.get(&next_loc).unwrap() {
                    *distance.get_mut(&next_loc).unwrap() = alt;
                    next_nodes.push(State {
                        dist: alt,
                        loc: next_loc,
                    });
                }
            }
        }
    }

    distance
}

fn solve2(input: &[&str], num_steps: usize) -> usize {
    let (start, grid) = parse(input);

    let distance_map = dijkstra_distance(&grid, start);

    let even_corners = distance_map
        .values()
        .filter(|v| **v != usize::MAX && **v > 65 && **v % 2 == 0)
        .count();
    let odd_corners = distance_map
        .values()
        .filter(|v| **v != usize::MAX && **v > 65 && **v % 2 == 1)
        .count();
    let even_full = distance_map
        .values()
        .filter(|v| **v != usize::MAX && **v % 2 == 0)
        .count();
    let odd_full = distance_map
        .values()
        .filter(|v| **v != usize::MAX && **v % 2 == 1)
        .count();

    let n = (num_steps - (grid.width() / 2)) / grid.width();

    ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners + n * even_corners
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input, 64));
    println!("part 2: {}", solve2(&input, 26501365));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "...........",
        ".....###.#.",
        ".###.##..#.",
        "..#.#...#..",
        "....#.#....",
        ".##..S####.",
        ".##..#...#.",
        ".......##..",
        ".##.#.####.",
        ".##..##.##.",
        "...........",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT, 6), 16)
    }
}
