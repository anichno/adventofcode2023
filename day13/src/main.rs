use utils::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GroundType {
    Ash,
    Rock,
}

type GroundGrid = Vec<Vec<GroundType>>;

fn parse(input: &[&str]) -> Vec<GroundGrid> {
    let mut maps = Vec::new();
    let mut grid = Vec::new();

    for line in input {
        if line.is_empty() {
            maps.push(grid);
            grid = Vec::new();
            continue;
        }
        let mut row = Vec::new();
        for chr in line.chars() {
            row.push(match chr {
                '.' => GroundType::Ash,
                '#' => GroundType::Rock,
                _ => panic!("invalid ground type: {chr}"),
            });
        }
        grid.push(row);
    }

    maps.push(grid);

    maps
}

fn solve1(input: &[&str]) -> u32 {
    let maps = parse(input);
    let mut total = 0;

    'outer: for map in maps {
        // check for mirror on rows
        for possible_horiz_mirror_idx in (0..map.len() - 1).filter(|i| map[*i] == map[*i + 1]) {
            // validate if this is the mirror for the full map
            if (0..possible_horiz_mirror_idx)
                .rev()
                .zip(possible_horiz_mirror_idx + 2..map.len())
                .all(|(l, r)| map[l] == map[r])
            {
                total += 100 * (possible_horiz_mirror_idx as u32 + 1);
                continue 'outer;
            }
        }

        // check for mirror on cols
        for possible_vert_mirror_idx in
            (0..map.width() - 1).filter(|i| map.get_column(*i) == map.get_column(*i + 1))
        {
            // validate if this is mirror for the full map
            if (0..possible_vert_mirror_idx)
                .rev()
                .zip(possible_vert_mirror_idx + 2..map.width())
                .all(|(l, r)| map.get_column(l) == map.get_column(r))
            {
                total += possible_vert_mirror_idx as u32 + 1;
                break;
            }
        }
    }

    total
}

fn solve2(input: &[&str]) -> u32 {
    let maps = parse(input);
    let mut total = 0;

    // Get original reflection lines
    let mut original_reflection_line = Vec::new();
    'outer: for map in maps.iter() {
        // check for mirror on rows
        for possible_horiz_mirror_idx in (0..map.len() - 1).filter(|i| map[*i] == map[*i + 1]) {
            // validate if this is the mirror for the full map
            if (0..possible_horiz_mirror_idx)
                .rev()
                .zip(possible_horiz_mirror_idx + 2..map.len())
                .all(|(l, r)| map[l] == map[r])
            {
                original_reflection_line.push((Some(possible_horiz_mirror_idx), None));
                continue 'outer;
            }
        }

        // check for mirror on cols
        for possible_vert_mirror_idx in
            (0..map.width() - 1).filter(|i| map.get_column(*i) == map.get_column(*i + 1))
        {
            // validate if this is mirror for the full map
            if (0..possible_vert_mirror_idx)
                .rev()
                .zip(possible_vert_mirror_idx + 2..map.width())
                .all(|(l, r)| map.get_column(l) == map.get_column(r))
            {
                original_reflection_line.push((None, Some(possible_vert_mirror_idx)));
                break;
            }
        }
    }

    'outer: for (map, orig_reflection_line) in maps.iter().zip(original_reflection_line.iter()) {
        for y in 0..map.len() {
            for x in 0..map.width() {
                // place smudge here
                let mut smudged_map = map.clone();
                if let GroundType::Ash = smudged_map[y][x] {
                    smudged_map[y][x] = GroundType::Rock;
                } else {
                    smudged_map[y][x] = GroundType::Ash;
                }

                // check for mirror on rows
                for possible_horiz_mirror_idx in
                    (0..smudged_map.len() - 1).filter(|i| smudged_map[*i] == smudged_map[*i + 1])
                {
                    // make sure this is a new reflection
                    if let Some(orig_horiz_line) = orig_reflection_line.0 {
                        if possible_horiz_mirror_idx == orig_horiz_line {
                            continue;
                        }
                    }
                    // validate if this is the mirror for the full smudged_map
                    if (0..possible_horiz_mirror_idx)
                        .rev()
                        .zip(possible_horiz_mirror_idx + 2..smudged_map.len())
                        .all(|(l, r)| smudged_map[l] == smudged_map[r])
                    {
                        total += 100 * (possible_horiz_mirror_idx as u32 + 1);
                        continue 'outer;
                    }
                }

                // check for mirror on cols
                for possible_vert_mirror_idx in (0..smudged_map.width() - 1).filter(|i| {
                    smudged_map.get_column(*i).is_some()
                        && smudged_map.get_column(*i + 1).is_some()
                        && smudged_map.get_column(*i) == smudged_map.get_column(*i + 1)
                }) {
                    // make sure this is a new reflection
                    if let Some(orig_vert_line) = orig_reflection_line.1 {
                        if possible_vert_mirror_idx == orig_vert_line {
                            continue;
                        }
                    }
                    // validate if this is mirror for the full smudged_map
                    if (0..possible_vert_mirror_idx)
                        .rev()
                        .zip(possible_vert_mirror_idx + 2..smudged_map.width())
                        .all(|(l, r)| {
                            smudged_map.get_column(l).is_some()
                                && smudged_map.get_column(r).is_some()
                                && smudged_map.get_column(l) == smudged_map.get_column(r)
                        })
                    {
                        total += possible_vert_mirror_idx as u32 + 1;
                        continue 'outer;
                    }
                }
            }
        }
    }

    total
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
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 405)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 400)
    }
}
