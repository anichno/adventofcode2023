use std::collections::{BinaryHeap, HashMap, HashSet};

use utils::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    last_dir: Direction,
    last_dir_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    node: Node,
    heat_loss: i64,
}

fn parse(input: &[&str]) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|d| d as i64).unwrap())
                .collect()
        })
        .collect()
}

fn dijkstra_distance_to(
    graph: &HashMap<Node, HashSet<Edge>>,
    source: Node,
    target: (usize, usize),
) -> Option<i64> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct State<'a> {
        heat_loss: i64,
        node: &'a Node,
    }

    impl<'a> Ord for State<'a> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.heat_loss.cmp(&self.heat_loss)
        }
    }

    impl<'a> PartialOrd for State<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut distance = HashMap::new();
    let mut next_nodes = BinaryHeap::new();

    for node in graph.keys() {
        distance.insert(node, i64::MAX);
    }
    next_nodes.push(State {
        heat_loss: 0,
        node: &source,
    });

    *distance.get_mut(&source).unwrap() = 0;

    while let Some(state) = next_nodes.pop() {
        let min_node = state.node;
        assert_ne!(*distance.get(min_node).unwrap(), i64::MAX);

        if min_node.x == target.0 && min_node.y == target.1 {
            return Some(*distance.get(min_node).unwrap());
        }

        if state.heat_loss > *distance.get(min_node).unwrap() {
            continue;
        }

        for edge in graph.get(min_node).unwrap() {
            let alt = state.heat_loss + edge.heat_loss;
            if alt < *distance.get(&edge.node).unwrap() {
                *distance.get_mut(&edge.node).unwrap() = alt;
                next_nodes.push(State {
                    heat_loss: alt,
                    node: &edge.node,
                });
            }
        }
    }

    None
}

mod part1 {
    use utils::{checked_offset, Grid};

    use super::*;
    use std::collections::HashMap;

    fn build_graph(grid: &[Vec<i64>]) -> HashMap<Node, HashSet<Edge>> {
        let mut nodes_to_visit = vec![Node {
            x: 0,
            y: 0,
            last_dir: Direction::Right,
            last_dir_count: 0,
        }];
        let mut graph = HashMap::new();
        while let Some(cur_node) = nodes_to_visit.pop() {
            graph.insert(cur_node, HashSet::new());

            for dir in match cur_node.last_dir {
                Direction::Up => [Direction::Left, Direction::Right, Direction::Up],
                Direction::Right => [Direction::Up, Direction::Right, Direction::Down],
                Direction::Down => [Direction::Left, Direction::Down, Direction::Right],
                Direction::Left => [Direction::Down, Direction::Left, Direction::Up],
            } {
                if dir == cur_node.last_dir && cur_node.last_dir_count == 3 {
                    continue;
                }

                let (x_offset, y_offset) = match dir {
                    Direction::Up => (0, -1),
                    Direction::Right => (1, 0),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                };

                if let (Some(new_x), Some(new_y)) = (
                    checked_offset(cur_node.x, x_offset),
                    checked_offset(cur_node.y, y_offset),
                ) {
                    if new_x < grid.width() && new_y < grid.height() {
                        let dir_count = if dir == cur_node.last_dir {
                            cur_node.last_dir_count + 1
                        } else {
                            1
                        };
                        let new_node = Node {
                            x: new_x,
                            y: new_y,
                            last_dir: dir,
                            last_dir_count: dir_count,
                        };
                        let heat_loss = grid[new_y][new_x];
                        graph.get_mut(&cur_node).unwrap().insert(Edge {
                            node: new_node,
                            heat_loss,
                        });

                        if let std::collections::hash_map::Entry::Vacant(e) = graph.entry(new_node)
                        {
                            e.insert(HashSet::new());
                            nodes_to_visit.push(new_node);
                        }
                    }
                }
            }
        }

        graph
    }

    pub fn solve1(input: &[&str]) -> i64 {
        let grid = parse(input);
        let graph = build_graph(&grid);

        dijkstra_distance_to(
            &graph,
            Node {
                x: 0,
                y: 0,
                last_dir: Direction::Right,
                last_dir_count: 0,
            },
            (grid.width() - 1, grid.height() - 1),
        )
        .unwrap()
    }
}

mod part2 {
    use utils::{checked_offset, Grid};

    use super::*;
    use std::collections::HashMap;

    fn build_graph(grid: &[Vec<i64>]) -> HashMap<Node, HashSet<Edge>> {
        let mut nodes_to_visit = vec![Node {
            x: 0,
            y: 0,
            last_dir: Direction::Right,
            last_dir_count: 0,
        }];
        let mut graph = HashMap::new();
        while let Some(cur_node) = nodes_to_visit.pop() {
            graph.insert(cur_node, HashSet::new());

            for dir in match cur_node.last_dir {
                Direction::Up => [Direction::Left, Direction::Right, Direction::Up],
                Direction::Right => [Direction::Up, Direction::Right, Direction::Down],
                Direction::Down => [Direction::Left, Direction::Down, Direction::Right],
                Direction::Left => [Direction::Down, Direction::Left, Direction::Up],
            } {
                if (dir == cur_node.last_dir && cur_node.last_dir_count == 10)
                    || (dir != cur_node.last_dir && cur_node.last_dir_count < 4)
                {
                    continue;
                }

                let (x_offset, y_offset) = match dir {
                    Direction::Up => (0, -1),
                    Direction::Right => (1, 0),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                };

                if let (Some(new_x), Some(new_y)) = (
                    checked_offset(cur_node.x, x_offset),
                    checked_offset(cur_node.y, y_offset),
                ) {
                    if new_x < grid.width() && new_y < grid.height() {
                        let dir_count = if dir == cur_node.last_dir {
                            cur_node.last_dir_count + 1
                        } else {
                            1
                        };
                        let new_node = Node {
                            x: new_x,
                            y: new_y,
                            last_dir: dir,
                            last_dir_count: dir_count,
                        };

                        let heat_loss = grid[new_y][new_x];
                        graph.get_mut(&cur_node).unwrap().insert(Edge {
                            node: new_node,
                            heat_loss,
                        });

                        if let std::collections::hash_map::Entry::Vacant(e) = graph.entry(new_node)
                        {
                            e.insert(HashSet::new());
                            nodes_to_visit.push(new_node);
                        }
                    }
                }
            }
        }

        graph
    }
    pub fn solve2(input: &[&str]) -> i64 {
        let grid = parse(input);
        let graph = build_graph(&grid);

        dijkstra_distance_to(
            &graph,
            Node {
                x: 0,
                y: 0,
                last_dir: Direction::Right,
                last_dir_count: 0,
            },
            (grid.width() - 1, grid.height() - 1),
        )
        .unwrap()
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
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ];

    #[test]
    fn test1() {
        assert_eq!(part1::solve1(INPUT), 102)
    }

    #[test]
    fn test2() {
        assert_eq!(part2::solve2(INPUT), 94)
    }
}
