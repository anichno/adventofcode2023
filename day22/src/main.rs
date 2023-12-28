use std::collections::{HashSet, VecDeque};

use utils::Point;

#[derive(Debug)]
struct Brick {
    top_z: usize,
    bottom_z: usize,
    z_diff: usize,
    xy_points: HashSet<Point>,
}

fn parse(input: &[&str]) -> Vec<Brick> {
    let mut bricks = Vec::new();

    for line in input {
        let (left, right) = line.split_once('~').unwrap();
        let mut left = left.split(',');
        let p1 = Point {
            x: left.next().unwrap().parse().unwrap(),
            y: left.next().unwrap().parse().unwrap(),
        };

        let mut right = right.split(',');
        let p2 = Point {
            x: right.next().unwrap().parse().unwrap(),
            y: right.next().unwrap().parse().unwrap(),
        };
        let left_z: usize = left.next().unwrap().parse().unwrap();
        let right_z = right.next().unwrap().parse().unwrap();
        let top_z = left_z.max(right_z);
        let bottom_z = left_z.min(right_z);
        let mut xy_points = HashSet::new();
        for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
            for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                xy_points.insert(Point { x, y });
            }
        }

        bricks.push(Brick {
            top_z,
            bottom_z,
            z_diff: top_z - bottom_z,
            xy_points,
        });
    }

    bricks
}

fn pack_bricks(bricks: &mut Vec<Brick>) {
    let mut moved_some = true;
    while moved_some {
        moved_some = false;
        for i in 0..bricks.len() {
            if bricks[i].bottom_z == 1 {
                // on ground, no shifts
                continue;
            }
            // find next brick with shared xy points with highest top_z
            let mut found_supporting_brick = false;
            let mut top_brick_idx = 0;
            let mut max_z = 0;
            for j in 0..bricks.len() {
                if i != j
                    && bricks[j].top_z < bricks[i].bottom_z
                    && bricks[j]
                        .xy_points
                        .intersection(&bricks[i].xy_points)
                        .next()
                        .is_some()
                    && bricks[j].top_z > max_z
                {
                    max_z = bricks[j].top_z;
                    top_brick_idx = j;
                    found_supporting_brick = true;
                }
            }

            if found_supporting_brick {
                if bricks[i].bottom_z - 1 != bricks[top_brick_idx].top_z {
                    moved_some = true;
                    bricks[i].bottom_z = bricks[top_brick_idx].top_z + 1;
                    bricks[i].top_z = bricks[i].bottom_z + bricks[i].z_diff;
                }
            } else {
                // on ground
                moved_some = true;
                bricks[i].bottom_z = 1;
                bricks[i].top_z = bricks[i].bottom_z + bricks[i].z_diff;
            }
        }
    }
    bricks.sort_unstable_by_key(|b| b.bottom_z);
}

fn solve1(input: &[&str]) -> i64 {
    let mut bricks = parse(input);
    pack_bricks(&mut bricks);

    let mut tot_remove = 0;
    for i in 0..bricks.len() {
        let mut supporting = Vec::new();
        for j in 0..bricks.len() {
            if i != j
                && bricks[j].bottom_z == bricks[i].top_z + 1
                && bricks[j]
                    .xy_points
                    .intersection(&bricks[i].xy_points)
                    .next()
                    .is_some()
            {
                supporting.push(j);
            }
        }
        if supporting.is_empty() {
            tot_remove += 1;
        } else {
            let mut all_supported = true;
            for supported in supporting {
                let mut alt_supporting = false;
                for j in 0..bricks.len() {
                    if i != j
                        && bricks[j].top_z == bricks[supported].bottom_z - 1
                        && bricks[j]
                            .xy_points
                            .intersection(&bricks[supported].xy_points)
                            .next()
                            .is_some()
                    {
                        alt_supporting = true;
                        break;
                    }
                }
                all_supported &= alt_supporting;
            }

            if all_supported {
                tot_remove += 1;
            }
        }
    }

    tot_remove
}

fn solve2(input: &[&str]) -> usize {
    let mut bricks = parse(input);
    pack_bricks(&mut bricks);

    let mut total = 0;

    let mut supported_by = vec![HashSet::new(); bricks.len()];
    let mut supporting = vec![HashSet::new(); bricks.len()];
    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if i == j {
                continue;
            }
            if bricks[i].bottom_z == bricks[j].top_z + 1
                && bricks[i]
                    .xy_points
                    .intersection(&bricks[j].xy_points)
                    .next()
                    .is_some()
            {
                supported_by[i].insert(j);
            } else if bricks[i].top_z == bricks[j].bottom_z - 1
                && bricks[i]
                    .xy_points
                    .intersection(&bricks[j].xy_points)
                    .next()
                    .is_some()
            {
                supporting[i].insert(j);
            }
        }
    }

    for remove_idx in 0..bricks.len() {
        let mut removed = HashSet::new();
        removed.insert(remove_idx);

        let mut to_check: VecDeque<usize> = VecDeque::new();
        to_check.extend(supporting[remove_idx].iter());
        while let Some(try_remove_idx) = to_check.pop_front() {
            if supported_by[try_remove_idx].is_subset(&removed) {
                removed.insert(try_remove_idx);
                to_check.extend(supporting[try_remove_idx].iter());
            }
        }
        total += removed.len() - 1;
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
        "1,0,1~1,2,1",
        "0,0,2~2,0,2",
        "0,2,3~2,2,3",
        "0,0,4~0,2,4",
        "2,0,5~2,2,5",
        "0,1,6~2,1,6",
        "1,1,8~1,1,9",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 5)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 7)
    }
}
