use utils::{checked_offset, Grid, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Ground,
    Start,
    Pipe(Option<Point>, Option<Point>),
}

fn parse(input: &[&str]) -> (Point, Vec<Vec<Location>>) {
    let mut grid = Vec::new();
    let mut start = Point { x: 0, y: 0 };
    for (y, line) in input.iter().enumerate() {
        let mut row = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            if chr == '.' {
                row.push(Location::Ground);
            } else if chr == 'S' {
                // resolve its connections later
                row.push(Location::Start);
                start = Point { x, y };
            } else {
                let ((x1, y1), (x2, y2)) = match chr {
                    '|' => ((0, -1), (0, 1)),
                    '-' => ((-1, 0), (1, 0)),
                    'L' => ((0, -1), (1, 0)),
                    'J' => ((-1, 0), (0, -1)),
                    '7' => ((-1, 0), (0, 1)),
                    'F' => ((0, 1), (1, 0)),
                    _ => panic!("unexpected char: {chr}"),
                };

                let conn1 = if let Some(x1) = checked_offset(x, x1) {
                    checked_offset(y, y1).map(|y1| Point { x: x1, y: y1 })
                } else {
                    None
                };

                let conn2 = if let Some(x2) = checked_offset(x, x2) {
                    checked_offset(y, y2).map(|y2| Point { x: x2, y: y2 })
                } else {
                    None
                };

                row.push(Location::Pipe(conn1, conn2));
            }
        }

        grid.push(row);
    }

    // resolve start connections
    let mut first = None;
    for adj in grid.adjacents(start.x, start.y) {
        if let Location::Pipe(Some(conn1), Some(conn2)) = adj.val {
            if (conn1.x == start.x && conn1.y == start.y)
                || (conn2.x == start.x && conn2.y == start.y)
            {
                if first.is_none() {
                    first = Some(Point { x: adj.x, y: adj.y });
                } else {
                    grid[start.y][start.x] =
                        Location::Pipe(first, Some(Point { x: adj.x, y: adj.y }));
                    break;
                }
            }
        }
    }

    let Location::Pipe(Some(_), Some(_)) = grid[start.y][start.x] else {
        panic!("Failed to connect start to pipes");
    };

    (start, grid)
}

fn solve1(input: &[&str]) -> u32 {
    let (start, grid) = parse(input);
    let Location::Pipe(Some(start_a), Some(start_b)) = grid[start.y][start.x] else {
        panic!()
    };
    let mut cur1 = start_a;
    let mut cur1_prev = start;

    let mut cur2 = start_b;
    let mut cur2_prev = start;

    let mut num_steps = 1;
    loop {
        if cur1 == cur2 {
            break;
        }

        let Location::Pipe(Some(next1_a), Some(next1_b)) = grid[cur1.y][cur1.x] else {
            panic!()
        };
        let next1 = if next1_a != cur1_prev {
            next1_a
        } else {
            next1_b
        };
        cur1_prev = cur1;
        cur1 = next1;

        let Location::Pipe(Some(next2_a), Some(next2_b)) = grid[cur2.y][cur2.x] else {
            panic!()
        };
        let next2 = if next2_a != cur2_prev {
            next2_a
        } else {
            next2_b
        };
        cur2_prev = cur2;
        cur2 = next2;

        num_steps += 1;
    }

    num_steps
}

fn solve2(input: &[&str]) -> u32 {
    let (start, grid) = parse(input);
    let Location::Pipe(Some(start_a), Some(_)) = grid[start.y][start.x] else {
        panic!()
    };

    let mut cur = start_a;
    let mut prev = start;

    // build new grid where any non main loop pipe is ground
    let mut main_loop_tiles = vec![vec![false; grid[0].len()]; grid.len()];
    main_loop_tiles[start.y][start.x] = true;
    while cur != start {
        main_loop_tiles[cur.y][cur.x] = true;

        let Location::Pipe(Some(next_a), Some(next_b)) = grid[cur.y][cur.x] else {
            panic!()
        };
        let next = if next_a != prev { next_a } else { next_b };

        prev = cur;
        cur = next;
    }

    let mut new_grid = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, col) in row.iter().enumerate() {
            if main_loop_tiles[y][x] {
                new_row.push(*col);
            } else {
                new_row.push(Location::Ground);
            }
        }
        new_grid.push(new_row);
    }

    let grid = new_grid;

    // double size of grid, tile is either pipe or not
    let mut doubled_grid = vec![vec![false; grid[0].len() * 2]; grid.len() * 2];
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            // translate old pipes
            if let Location::Pipe(Some(conn_a), Some(conn_b)) = *col {
                let (new_x, new_y) = (x * 2, y * 2);
                doubled_grid[new_y][new_x] = true;

                let (a_offset_x, a_offset_y) =
                    (conn_a.x as i32 - x as i32, conn_a.y as i32 - y as i32);

                if let Some(middle_x) = checked_offset(new_x, a_offset_x) {
                    doubled_grid[new_y][middle_x] = true;
                }
                if let Some(middle_y) = checked_offset(new_y, a_offset_y) {
                    doubled_grid[middle_y][new_x] = true;
                }

                let (b_offset_x, b_offset_y) =
                    (conn_b.x as i32 - x as i32, conn_b.y as i32 - y as i32);
                if let Some(middle_x) = checked_offset(new_x, b_offset_x) {
                    doubled_grid[new_y][middle_x] = true;
                }
                if let Some(middle_y) = checked_offset(new_y, b_offset_y) {
                    doubled_grid[middle_y][new_x] = true;
                }
            }
        }
    }

    // add ground tiles to full perimeter
    doubled_grid.insert(0, vec![false; doubled_grid[0].len()]);
    doubled_grid.push(vec![false; doubled_grid[0].len()]);
    for row in doubled_grid.iter_mut() {
        row.insert(0, false);
        row.push(false);
    }

    let mut outside_tiles = vec![vec![false; doubled_grid[0].len()]; doubled_grid.len()];

    // flood fill outside tiles, bounded by pipes
    let mut seeds = vec![Point { x: 0, y: 0 }];

    while let Some(seed) = seeds.pop() {
        for adj in doubled_grid.adjacents(seed.x, seed.y) {
            if !doubled_grid[adj.y][adj.x] && !outside_tiles[adj.y][adj.x] {
                outside_tiles[adj.y][adj.x] = true;
                seeds.push(Point { x: adj.x, y: adj.y });
            }
        }
    }

    let mut num_inside = 0;
    for (y, row) in outside_tiles.iter().enumerate().skip(1).step_by(2) {
        for (x, col) in row.iter().enumerate().skip(1).step_by(2) {
            if !doubled_grid[y][x] && !col {
                num_inside += 1;
            }
        }
    }

    num_inside
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &[&str] = &[".....", ".S-7.", ".|.|.", ".L-J.", "....."];
    const INPUT2: &[&str] = &["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];
    const INPUT3: &[&str] = &[
        "...........",
        ".S-------7.",
        ".|F-----7|.",
        ".||.....||.",
        ".||.....||.",
        ".|L-7.F-J|.",
        ".|..|.|..|.",
        ".L--J.L--J.",
        "...........",
    ];
    const INPUT4: &[&str] = &[
        ".F----7F7F7F7F-7....",
        ".|F--7||||||||FJ....",
        ".||.FJ||||||||L7....",
        "FJL7L7LJLJ||LJ.L-7..",
        "L--J.L7...LJS7F-7L7.",
        "....F-J..F7FJ|L7L7L7",
        "....L7.F7||L7|.L7L7|",
        ".....|FJLJ|FJ|F7|.LJ",
        "....FJL-7.||.||||...",
        "....L---J.LJ.LJLJ...",
    ];
    const INPUT5: &[&str] = &[
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT1), 4);
        assert_eq!(solve1(INPUT2), 8);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT3), 4);
        assert_eq!(solve2(INPUT4), 8);
        assert_eq!(solve2(INPUT5), 10);
    }
}
