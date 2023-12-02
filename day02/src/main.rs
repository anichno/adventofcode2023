#[derive(Debug, PartialEq, Eq)]
struct GameSubset {
    r: u32,
    g: u32,
    b: u32,
}

fn parse_game(input: &str) -> (u32, Vec<GameSubset>) {
    let mut subsets = Vec::new();
    let (_, cur_line) = input.split_once(' ').unwrap();
    let (game_id, cur_line) = cur_line.split_once(':').unwrap();
    let game_id = game_id.parse().unwrap();

    for subset in cur_line.split(';') {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for color_sample in subset.split(',').map(|c| c.trim()) {
            let (num, color) = color_sample.split_once(' ').unwrap();
            let num = num.parse().unwrap();
            match color {
                "red" => r = num,
                "green" => g = num,
                "blue" => b = num,
                _ => panic!("{} not a valid color", color),
            }
        }

        subsets.push(GameSubset { r, g, b });
    }

    (game_id, subsets)
}

fn solve1(input: &[&str]) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut valid_games_sum = 0;

    for line in input {
        let (game_id, subsets) = parse_game(line);
        let mut all_valid = true;
        for subset in subsets {
            if subset.r > max_red || subset.g > max_green || subset.b > max_blue {
                all_valid = false;
                break;
            }
        }

        if all_valid {
            valid_games_sum += game_id;
        }
    }

    valid_games_sum
}

fn solve2(input: &[&str]) -> u32 {
    let mut sum = 0;

    for line in input {
        let (_, subsets) = parse_game(line);
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for subset in subsets {
            min_red = min_red.max(subset.r);
            min_green = min_green.max(subset.g);
            min_blue = min_blue.max(subset.b);
        }

        let cube_power = min_red * min_green * min_blue;
        sum += cube_power;
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
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn parse() {
        let (game_id, subsets) = parse_game(INPUT[0]);
        assert_eq!(game_id, 1);
        assert_eq!(
            &subsets,
            &[
                GameSubset { r: 4, g: 0, b: 3 },
                GameSubset { r: 1, g: 2, b: 6 },
                GameSubset { r: 0, g: 2, b: 0 }
            ]
        )
    }

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 8)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(&INPUT[0..1]), 48);
        assert_eq!(solve2(INPUT), 2286)
    }
}
