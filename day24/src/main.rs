use std::fmt::Display;

use z3::{
    ast::{Ast, Bool, Int, Real},
    Config, Context, Solver,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn xy_angle(&self, other: &Self) -> f64 {
        let x = (other.x - self.x) as f64;
        let y = (other.y - self.y) as f64;

        y.atan2(x).to_degrees()
    }
}

#[derive(Debug, Clone, Copy)]
struct XYLine {
    p1: Point,
    p2: Point,
    m: f64,
    b: f64,
}

impl XYLine {
    fn new(p1: Point, p2: Point) -> Self {
        let x1 = p1.x as f64;
        let y1 = p1.y as f64;
        let x2 = p2.x as f64;
        let y2 = p2.y as f64;

        let m = (y2 - y1) / (x2 - x1);
        let b = y1 - m * x1;

        Self { p1, p2, m, b }
    }
    fn intersection(&self, other: &Self) -> Option<Point> {
        if self.m == other.m {
            // Parallel or coincident
            return None;
        }

        let x: f64;
        let y: f64;

        if self.m.is_infinite() {
            x = self.p1.x as f64;
            y = other.m * x + other.b;
        } else if other.m.is_infinite() {
            x = other.p1.x as f64;
            y = self.m * x + self.b;
        } else {
            x = (other.b - self.b) / (self.m - other.m);
            y = self.m * x + self.b;
        }

        Some(Point {
            x: x.round() as i64,
            y: y.round() as i64,
            z: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hailstone {
    position: Point,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.position.x, self.position.y, self.position.z, self.vx, self.vy, self.vz
        )
    }
}

fn parse(input: &[&str]) -> Vec<Hailstone> {
    input
        .iter()
        .map(|line| {
            let (pos_line, vel_line) = line.split_once(" @ ").unwrap();
            let mut positions = pos_line.split(", ");
            let x = positions.next().unwrap().trim().parse().unwrap();
            let y = positions.next().unwrap().trim().parse().unwrap();
            let z = positions.next().unwrap().trim().parse().unwrap();

            let mut velocities = vel_line.split(", ");
            let vx = velocities.next().unwrap().trim().parse().unwrap();
            let vy = velocities.next().unwrap().trim().parse().unwrap();
            let vz = velocities.next().unwrap().trim().parse().unwrap();

            Hailstone {
                position: Point { x, y, z },
                vx,
                vy,
                vz,
            }
        })
        .collect()
}

fn solve1(input: &[&str], range_min: i64, range_max: i64) -> i64 {
    let hailstones = parse(input);
    let hailstone_paths: Vec<(Hailstone, XYLine)> = hailstones
        .iter()
        .map(|h| {
            let p2 = Point {
                x: h.position.x + h.vx,
                y: h.position.y + h.vy,
                z: h.position.z + h.vz,
            };
            let line = XYLine::new(h.position, p2);
            (*h, line)
        })
        .collect();

    let mut future_intersections = 0;
    for (i, (hstone1, hpath1)) in hailstone_paths.iter().enumerate() {
        for (hstone2, hpath2) in hailstone_paths.iter().skip(i + 1) {
            if hstone1 == hstone2 {
                continue;
            }
            if let Some(intersect) = hpath1.intersection(hpath2) {
                if intersect.x >= range_min
                    && intersect.x <= range_max
                    && intersect.y >= range_min
                    && intersect.y <= range_max
                    && (hpath1.p1.xy_angle(&hpath1.p2) - hpath1.p1.xy_angle(&intersect)).abs()
                        < 10.0
                    && (hpath2.p1.xy_angle(&hpath2.p2) - hpath2.p1.xy_angle(&intersect)).abs()
                        < 10.0
                {
                    future_intersections += 1;
                }
            }
        }
    }

    future_intersections
}

fn solve2(input: &[&str]) -> i64 {
    let hailstones = parse(input);

    let mut config = Config::new();
    config.set_model_generation(true);
    let context = Context::new(&config);
    let solver = Solver::new(&context);

    let rock_x = Real::new_const(&context, "rock_x");
    let rock_y = Real::new_const(&context, "rock_y");
    let rock_z = Real::new_const(&context, "rock_z");
    let rock_dx = Real::new_const(&context, "rock_dx");
    let rock_dy = Real::new_const(&context, "rock_dy");
    let rock_dz = Real::new_const(&context, "rock_dz");

    let constraints = hailstones
        .iter()
        .enumerate()
        .map(|(t, h)| {
            let time = Real::new_const(&context, format!("t{t}"));
            (rock_x.clone() + rock_dx.clone() * time.clone())._eq(
                &(Int::from_i64(&context, h.position.x).to_real()
                    + Int::from_i64(&context, h.vx).to_real() * time.clone()),
            ) & (rock_y.clone() + rock_dy.clone() * time.clone())._eq(
                &(&Int::from_i64(&context, h.position.y).to_real()
                    + Int::from_i64(&context, h.vy).to_real() * time.clone()),
            ) & (rock_z.clone() + rock_dz.clone() * time.clone())._eq(
                &(Int::from_i64(&context, h.position.z).to_real()
                    + Int::from_i64(&context, h.vz).to_real() * time.clone()),
            )
        })
        .fold(Bool::from_bool(&context, true), |a, o| a & o);

    solver.assert(&constraints);
    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let x = model.eval(&rock_x, true).unwrap().as_real().unwrap().0;
            let y = model.eval(&rock_y, true).unwrap().as_real().unwrap().0;
            let z = model.eval(&rock_z, true).unwrap().as_real().unwrap().0;
            x + y + z
        }
        _ => panic!("failed to sat"),
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!(
        "part 1: {}",
        solve1(&input, 200000000000000, 400000000000000)
    );
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "19, 13, 30 @ -2,  1, -2",
        "18, 19, 22 @ -1, -1, -2",
        "20, 25, 34 @ -2, -2, -4",
        "12, 31, 28 @ -1, -2, -1",
        "20, 19, 15 @  1, -5, -3",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT, 7, 27), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 47)
    }
}
