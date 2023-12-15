fn solve1(input: &[&str]) -> i32 {
    let mut galaxies = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }

    galaxies.sort_unstable_by_key(|v| v.0);
    let mut prev_x = 0;
    let mut cur_x_push = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.0 != prev_x {
            cur_x_push += galaxy.0 - prev_x - 1;
            prev_x = galaxy.0;
        }
        galaxy.0 += cur_x_push;
    }

    galaxies.sort_unstable_by_key(|v| v.1);
    let mut prev_y = 0;
    let mut cur_y_push = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.1 != prev_y {
            cur_y_push += galaxy.1 - prev_y - 1;
            prev_y = galaxy.1;
        }
        galaxy.1 += cur_y_push;
    }

    // find shortest path between all galaxies
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }

    sum
}

fn solve2(input: &[&str], scale_factor: i64) -> i64 {
    let mut galaxies = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }

    galaxies.sort_unstable_by_key(|v| v.0);
    let mut prev_x = 0;
    let mut cur_x_push = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.0 != prev_x {
            cur_x_push += (galaxy.0 - prev_x - 1) * (scale_factor - 1);
            prev_x = galaxy.0;
        }
        galaxy.0 += cur_x_push;
    }

    galaxies.sort_unstable_by_key(|v| v.1);
    let mut prev_y = 0;
    let mut cur_y_push = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.1 != prev_y {
            cur_y_push += (galaxy.1 - prev_y - 1) * (scale_factor - 1);
            prev_y = galaxy.1;
        }
        galaxy.1 += cur_y_push;
    }

    // find shortest path between all galaxies
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }

    sum
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 374)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT, 10), 1030);
        assert_eq!(solve2(INPUT, 100), 8410);
    }
}
