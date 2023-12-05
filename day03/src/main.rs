use std::collections::HashSet;

use utils::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridNum {
    val: u32,
    num_chars: usize,
    id: usize,
}

#[derive(Debug, Clone)]
enum Token {
    Empty,
    Symbol(char),
    Number(GridNum),
}

fn parse(input: &[&str]) -> Vec<Vec<Token>> {
    fn resolve_buffered_numbers(num_buf: &mut Vec<u32>, row: &mut Vec<Token>, id: usize) {
        let mut num = 0;
        for digit in num_buf.iter() {
            num = num * 10 + digit;
        }

        for _ in 0..num_buf.len() {
            row.push(Token::Number(GridNum {
                val: num,
                num_chars: num_buf.len(),
                id,
            }));
        }
        num_buf.clear();
    }

    let mut num_id = 0;
    let mut grid = Vec::new();
    for line in input {
        let mut row = Vec::new();
        let mut num_buf = Vec::new();
        for chr in line.chars() {
            if chr.is_ascii_digit() {
                num_buf.push(chr.to_digit(10).unwrap())
            } else {
                if !num_buf.is_empty() {
                    resolve_buffered_numbers(&mut num_buf, &mut row, num_id);
                    num_id += 1;
                }
                if chr == '.' {
                    row.push(Token::Empty);
                } else {
                    row.push(Token::Symbol(chr));
                }
            }
        }
        if !num_buf.is_empty() {
            resolve_buffered_numbers(&mut num_buf, &mut row, num_id);
            num_id += 1;
        }

        grid.push(row);
    }

    grid
}

fn solve1(input: &[&str]) -> u32 {
    let grid = parse(input);
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        let mut cur_num_id = None;
        for (x, col) in row.iter().enumerate() {
            if let Token::Number(grid_num) = col {
                if cur_num_id.is_some() {
                    continue;
                }
                cur_num_id = Some(grid_num.id);
                if (x..x + grid_num.num_chars).any(|x_span| {
                    grid.adjacents(x_span, y)
                        .any(|t| matches!(t.val, Token::Symbol(_)))
                }) {
                    sum += grid_num.val;
                }
            } else {
                cur_num_id = None;
            }
        }
    }

    sum
}

fn solve2(input: &[&str]) -> u32 {
    let grid = parse(input);
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if let Token::Symbol(sym) = col {
                if *sym == '*' {
                    let mut adjacent_grid_nums = HashSet::new();
                    for adj in grid.adjacents(x, y) {
                        if let Token::Number(grid_num) = adj.val {
                            adjacent_grid_nums.insert(grid_num);
                        }
                    }
                    if adjacent_grid_nums.len() == 2 {
                        sum += adjacent_grid_nums.into_iter().fold(1, |a, e| a * e.val);
                    }
                }
            }
        }
    }

    sum
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
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 4361)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 467835)
    }
}
