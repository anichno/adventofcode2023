use utils::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    RoundRock,
    CubeRock,
    Ground,
}

fn parse(input: &[&str]) -> Vec<Vec<TileType>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => TileType::RoundRock,
                    '#' => TileType::CubeRock,
                    '.' => TileType::Ground,
                    _ => panic!("invalid char: {c}"),
                })
                .collect()
        })
        .collect()
}

fn solve1(input: &[&str]) -> u32 {
    let mut grid = parse(input);

    // tilt to north
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let TileType::RoundRock = grid[y][x] {
                // roll it up
                let rock_new_y = (0..y)
                    .rev()
                    .find(|cy| !matches!(grid[*cy][x], TileType::Ground))
                    .map(|ny| ny + 1)
                    .unwrap_or_default();
                grid[y][x] = TileType::Ground;
                grid[rock_new_y][x] = TileType::RoundRock;
            }
        }
    }

    // calc total load
    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        let num_round = row
            .iter()
            .filter(|t| matches!(t, TileType::RoundRock))
            .count();
        total += num_round * (grid.height() - y);
    }

    total as u32
}

fn solve2(input: &[&str]) -> u32 {
    const TOTAL_CYCLES: usize = 1000000000;
    let mut grid = parse(input);

    let mut last_loads = [(0, 0); 1000];
    for cycle in 0..TOTAL_CYCLES {
        // tilt to north
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if let TileType::RoundRock = grid[y][x] {
                    // roll it up
                    let rock_new_y = (0..y)
                        .rev()
                        .find(|cy| !matches!(grid[*cy][x], TileType::Ground))
                        .map(|ny| ny + 1)
                        .unwrap_or_default();
                    grid[y][x] = TileType::Ground;
                    grid[rock_new_y][x] = TileType::RoundRock;
                }
            }
        }

        // tilt west
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if let TileType::RoundRock = grid[y][x] {
                    // roll it left
                    let rock_new_x = (0..x)
                        .rev()
                        .find(|cx| !matches!(grid[y][*cx], TileType::Ground))
                        .map(|nx| nx + 1)
                        .unwrap_or_default();
                    grid[y][x] = TileType::Ground;
                    grid[y][rock_new_x] = TileType::RoundRock;
                }
            }
        }

        // tilt south
        for y in (0..grid.height()).rev() {
            for x in 0..grid.width() {
                if let TileType::RoundRock = grid[y][x] {
                    // roll it down
                    let rock_new_y = (y + 1..grid.height())
                        .find(|cy| !matches!(grid[*cy][x], TileType::Ground))
                        .map(|ny| ny - 1)
                        .unwrap_or(grid.height() - 1);
                    grid[y][x] = TileType::Ground;
                    grid[rock_new_y][x] = TileType::RoundRock;
                }
            }
        }

        // tilt east
        for y in 0..grid.height() {
            for x in (0..grid.width()).rev() {
                if let TileType::RoundRock = grid[y][x] {
                    // roll it right
                    let rock_new_x = (x + 1..grid.width())
                        .find(|cx| !matches!(grid[y][*cx], TileType::Ground))
                        .map(|nx| nx - 1)
                        .unwrap_or(grid.width() - 1);
                    grid[y][x] = TileType::Ground;
                    grid[y][rock_new_x] = TileType::RoundRock;
                }
            }
        }

        // calc total load
        let mut total = 0;
        for (y, row) in grid.iter().enumerate() {
            let num_round = row
                .iter()
                .filter(|t| matches!(t, TileType::RoundRock))
                .count();
            total += (num_round * (grid.height() - y)) as u32;
        }

        last_loads.rotate_left(1);
        last_loads[last_loads.len() - 1] = (cycle, total);

        // search for cycles in last_loads
        if cycle > last_loads.len() {
            for offset in 0..last_loads.len() - 1 {
                for cycle_size in 1..(last_loads.len() - 1 - offset) {
                    if last_loads[offset].1 == last_loads[offset + cycle_size].1
                        && last_loads[offset + 1].1 == last_loads[offset + cycle_size + 1].1
                    {
                        return last_loads[offset..offset + cycle_size]
                            [(TOTAL_CYCLES - last_loads[offset].0 - 1) % cycle_size]
                            .1;
                    }
                }
            }
        }
    }

    last_loads[0].1
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
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 136)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 64)
    }
}
