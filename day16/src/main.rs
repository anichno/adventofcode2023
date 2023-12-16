use std::collections::HashSet;

use utils::{Direction, Grid, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Empty,
    Mirror(MirrorQuadrant),
    Splitter(SplitDirection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorQuadrant {
    NorthEast,
    NorthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SplitDirection {
    Horizontal,
    Vertical,
}

fn reflect(light_dir: Direction, mirror: MirrorQuadrant) -> Direction {
    match (light_dir, mirror) {
        (Direction::Up, MirrorQuadrant::NorthEast) => Direction::Left,
        (Direction::Up, MirrorQuadrant::NorthWest) => Direction::Right,
        (Direction::Right, MirrorQuadrant::NorthEast) => Direction::Down,
        (Direction::Right, MirrorQuadrant::NorthWest) => Direction::Up,
        (Direction::Down, MirrorQuadrant::NorthEast) => Direction::Right,
        (Direction::Down, MirrorQuadrant::NorthWest) => Direction::Left,
        (Direction::Left, MirrorQuadrant::NorthEast) => Direction::Up,
        (Direction::Left, MirrorQuadrant::NorthWest) => Direction::Down,
    }
}

fn split(light_dir: Direction, splitter: SplitDirection) -> (Direction, Option<Direction>) {
    match (light_dir, splitter) {
        (Direction::Up | Direction::Down, SplitDirection::Horizontal) => {
            (Direction::Left, Some(Direction::Right))
        }
        (Direction::Up | Direction::Down, SplitDirection::Vertical) => (light_dir, None),
        (Direction::Left | Direction::Right, SplitDirection::Horizontal) => (light_dir, None),
        (Direction::Left | Direction::Right, SplitDirection::Vertical) => {
            (Direction::Up, Some(Direction::Down))
        }
    }
}

fn move_light(
    light_dir: Direction,
    light_location: Point,
    grid_width: usize,
    grid_height: usize,
) -> Option<Point> {
    match light_dir {
        Direction::Up => light_location.y.checked_sub(1).map(|new_y| Point {
            x: light_location.x,
            y: new_y,
        }),
        Direction::Right => {
            let new_x = light_location.x + 1;
            if new_x < grid_width {
                Some(Point {
                    x: new_x,
                    y: light_location.y,
                })
            } else {
                None
            }
        }
        Direction::Down => {
            let new_y = light_location.y + 1;
            if new_y < grid_height {
                Some(Point {
                    x: light_location.x,
                    y: new_y,
                })
            } else {
                None
            }
        }
        Direction::Left => light_location.x.checked_sub(1).map(|new_x| Point {
            x: new_x,
            y: light_location.y,
        }),
    }
}

fn parse(input: &[&str]) -> Vec<Vec<TileType>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => TileType::Empty,
                    '/' => TileType::Mirror(MirrorQuadrant::NorthWest),
                    '\\' => TileType::Mirror(MirrorQuadrant::NorthEast),
                    '-' => TileType::Splitter(SplitDirection::Horizontal),
                    '|' => TileType::Splitter(SplitDirection::Vertical),
                    _ => panic!("invalid tile: {c}"),
                })
                .collect()
        })
        .collect()
}

fn solve1(input: &[&str]) -> i64 {
    let grid = parse(input);
    let mut lit_tiles = vec![vec![false; grid.width()]; grid.height()];
    let mut cur_lights = vec![(Direction::Right, Point { x: 0, y: 0 })];
    let mut light_loop_detect = HashSet::new();
    while let Some((light_dir, light_loc)) = cur_lights.pop() {
        if !light_loop_detect.insert((light_dir, light_loc)) {
            continue;
        }

        lit_tiles[light_loc.y][light_loc.x] = true;
        match grid[light_loc.y][light_loc.x] {
            TileType::Empty => {
                if let Some(light_point) =
                    move_light(light_dir, light_loc, grid.width(), grid.height())
                {
                    cur_lights.push((light_dir, light_point));
                }
            }
            TileType::Mirror(mirror) => {
                let new_light_dir = reflect(light_dir, mirror);
                if let Some(light_point) =
                    move_light(new_light_dir, light_loc, grid.width(), grid.height())
                {
                    cur_lights.push((new_light_dir, light_point));
                }
            }
            TileType::Splitter(splitter) => {
                let (light_dir1, light_dir2) = split(light_dir, splitter);
                if let Some(light_point) =
                    move_light(light_dir1, light_loc, grid.width(), grid.height())
                {
                    cur_lights.push((light_dir1, light_point));
                }

                if let Some(light_dir2) = light_dir2 {
                    if let Some(light_point) =
                        move_light(light_dir2, light_loc, grid.width(), grid.height())
                    {
                        cur_lights.push((light_dir2, light_point));
                    }
                }
            }
        }
    }

    lit_tiles
        .iter()
        .map(|r| r.iter().filter(|i| **i).count())
        .sum::<usize>() as i64
}

fn solve2(input: &[&str]) -> i64 {
    let grid = parse(input);
    let mut max_illuminated = 0;
    for (start_dir, start_point) in (0..grid.width())
        .flat_map(|x| {
            [
                (Direction::Down, Point { x, y: 0 }),
                (
                    Direction::Up,
                    Point {
                        x,
                        y: grid.height() - 1,
                    },
                ),
            ]
        })
        .chain((0..grid.height()).flat_map(|y| {
            [
                (
                    Direction::Left,
                    Point {
                        x: grid.width() - 1,
                        y,
                    },
                ),
                (Direction::Right, Point { x: 0, y }),
            ]
        }))
    {
        let mut lit_tiles = vec![vec![false; grid.width()]; grid.height()];
        let mut cur_lights = vec![(start_dir, start_point)];
        let mut light_loop_detect = HashSet::new();
        while let Some((light_dir, light_loc)) = cur_lights.pop() {
            if !light_loop_detect.insert((light_dir, light_loc)) {
                continue;
            }

            lit_tiles[light_loc.y][light_loc.x] = true;
            match grid[light_loc.y][light_loc.x] {
                TileType::Empty => {
                    if let Some(light_point) =
                        move_light(light_dir, light_loc, grid.width(), grid.height())
                    {
                        cur_lights.push((light_dir, light_point));
                    }
                }
                TileType::Mirror(mirror) => {
                    let new_light_dir = reflect(light_dir, mirror);
                    if let Some(light_point) =
                        move_light(new_light_dir, light_loc, grid.width(), grid.height())
                    {
                        cur_lights.push((new_light_dir, light_point));
                    }
                }
                TileType::Splitter(splitter) => {
                    let (light_dir1, light_dir2) = split(light_dir, splitter);
                    if let Some(light_point) =
                        move_light(light_dir1, light_loc, grid.width(), grid.height())
                    {
                        cur_lights.push((light_dir1, light_point));
                    }

                    if let Some(light_dir2) = light_dir2 {
                        if let Some(light_point) =
                            move_light(light_dir2, light_loc, grid.width(), grid.height())
                        {
                            cur_lights.push((light_dir2, light_point));
                        }
                    }
                }
            }
        }

        max_illuminated = max_illuminated.max(
            lit_tiles
                .iter()
                .map(|r| r.iter().filter(|i| **i).count())
                .sum::<usize>() as i64,
        );
    }

    max_illuminated
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
        r#".|...\...."#,
        r#"|.-.\....."#,
        r#".....|-..."#,
        r#"........|."#,
        r#".........."#,
        r#".........\"#,
        r#"..../.\\.."#,
        r#".-.-/..|.."#,
        r#".|....-|.\"#,
        r#"..//.|...."#,
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 46)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 51)
    }
}
