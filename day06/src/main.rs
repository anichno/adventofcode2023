#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

fn solve1(input: &[Race]) -> u32 {
    let mut total = 1;
    for race in input {
        let mut num_ways = 0;
        for hold_time in 1..race.time {
            if (race.time - hold_time) * hold_time > race.distance {
                num_ways += 1;
            }
        }
        total *= num_ways
    }

    total
}

fn solve2(race: Race) -> u64 {
    let mut num_ways = 0;
    for hold_time in 1..race.time {
        if (race.time - hold_time) * hold_time > race.distance {
            num_ways += 1;
        }
    }

    num_ways
}

fn main() {
    // let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let input = &[
        Race {
            time: 44,
            distance: 283,
        },
        Race {
            time: 70,
            distance: 1134,
        },
        Race {
            time: 70,
            distance: 1134,
        },
        Race {
            time: 80,
            distance: 1491,
        },
    ];

    println!("part 1: {}", solve1(input));
    println!(
        "part 2: {}",
        solve2(Race {
            time: 44707080,
            distance: 283113411341491
        })
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[Race] = &[
        Race {
            time: 7,
            distance: 9,
        },
        Race {
            time: 15,
            distance: 40,
        },
        Race {
            time: 30,
            distance: 200,
        },
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 288)
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve2(Race {
                time: 71530,
                distance: 940200
            }),
            71503
        )
    }
}
