use num::bigint::ToBigInt;
use num::ToPrimitive;

pub fn solve() {
    let data = std::fs::read_to_string("data/day24.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data, 200000000000000, 400000000000000));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str, start: u64, stop: u64) -> u64 {
    let stones: Vec<_> = data.lines().map(|l| Stone::parse(l)).collect();
    let range = start as f64..=stop as f64;

    // process all pairs of lines and count the collisions
    let mut count = 0;
    for (i, s0) in stones.iter().enumerate() {
        for s1 in &stones[0..i] {
            // convert the pair of lines into a system of 2 equations to find the collision time
            //    x = v0x*s + p0x
            //    x = v1x*t + p1x
            //    => v0x*s + -v1x*t = p1x - p0x
            // the same thing can be done with y to create the second equation
            let eq = vec![
                vec![s0.v.x as f64, -s1.v.x as f64, s1.p.x as f64 - s0.p.x as f64],
                vec![s0.v.y as f64, -s1.v.y as f64, s1.p.y as f64 - s0.p.y as f64],
            ];

            // solve the linear system and verify times are not in the past
            let v = linear_solve(eq);
            let s = v[0];
            let t = v[1];
            if s < 0.0 || t < 0.0 {
                continue;
            }

            // calculate results and verify they are in the target range
            let x = s0.v.x as f64 * s + s0.p.x as f64;
            let y = s0.v.y as f64 * s + s0.p.y as f64;
            if range.contains(&x) && range.contains(&y) {
                count += 1
            }
        }
    }

    count
}

fn part2(data: &str) -> u64 {
    let s: Vec<_> = data.lines().map(|l| Stone::parse(l)).collect();

    // here we set up a system of equations to find the rock initial position P<xyz> and velocity V<xyz>
    // we know that at time t, the positions of the rock (P/V) and all hailstones (p/v) will be equal:
    //    x1 = vx1*t + px1
    //    x1 = Vx*t + Px
    //    => vx1*t + px1 = Vx*t + Px
    // we can rearrange this eliminate the t variable
    //    t = (Px - px1) / (vx1 - Vx)
    //    t = (Py - py1) / (vy1 - Vy)
    //    => (Px - px1) * (vy1 - Vy) = (Py - py1) * (vx1 - Vx)
    // we can now distribute and move the nonlinear parts that don't depend on hailstones to one side
    //    Px*vy1 - Px*Vy - px1*vy1 + px1*Vy = Py*vx1 - Py*Vx - py1*vx1 + py1*Vx
    //    => Py*Vx - Px*Vy = Py*vx1 - Px*vy1 + py1*Vx - px1*Vy + px1*vy1 - py1*vx1
    // this lhs is the same for every hailstone, so we can pick two
    //    Py*vx1 - Px*vy1 + py1*Vx - px1*Vy + px1*vy1 - py1*vx1 = Py*vx2 - Px*vy2 + py2*Vx - px2*Vy + px2*vy2 - py2*vx2
    // moving all of the goal variables to one side:
    //    Px*(vy2 - vy1) + Py*(vx1 - vx2) + Vx*(py1 - py2) + Vy*(px2 - px1) = px2*vy2 - py2*vx2 + py1*vx1 - px1*vy1
    // gives us a single equation for Px/Py/Vx/Vy, which we can repeat for 3 more data points to get
    // a system of 4 equations for the 4 variables
    //    [(vy2 - vy1) (vx1 - vx2) (py1 - py2) (px2 - px1)] [Px] = [px2*vy2 - py2*vx2 + py1*vx1 - px1*vy1]
    //    [(vy3 - vy1) (vx1 - vx3) (py1 - py3) (px3 - px1)] [Py] = [px3*vy3 - py3*vx3 + py1*vx1 - px1*vy1]
    //    [(vy4 - vy1) (vx1 - vx4) (py1 - py4) (px4 - px1)] [Vx] = [px4*vy4 - py4*vx4 + py1*vx1 - px1*vy1]
    //    [(vy5 - vy1) (vx1 - vx5) (py1 - py5) (px5 - px1)] [Vy] = [px5*vy5 - py5*vx5 + py1*vx1 - px1*vy1]
    // solving this system gives us the final Px/Py data points
    let eqxy = [
        [
            s[1].v.y - s[0].v.y,
            s[0].v.x - s[1].v.x,
            s[0].p.y - s[1].p.y,
            s[1].p.x - s[0].p.x,
            s[1].p.x * s[1].v.y - s[1].p.y * s[1].v.x + s[0].p.y * s[0].v.x - s[0].p.x * s[0].v.y,
        ],
        [
            s[2].v.y - s[0].v.y,
            s[0].v.x - s[2].v.x,
            s[0].p.y - s[2].p.y,
            s[2].p.x - s[0].p.x,
            s[2].p.x * s[2].v.y - s[2].p.y * s[2].v.x + s[0].p.y * s[0].v.x - s[0].p.x * s[0].v.y,
        ],
        [
            s[3].v.y - s[0].v.y,
            s[0].v.x - s[3].v.x,
            s[0].p.y - s[3].p.y,
            s[3].p.x - s[0].p.x,
            s[3].p.x * s[3].v.y - s[3].p.y * s[3].v.x + s[0].p.y * s[0].v.x - s[0].p.x * s[0].v.y,
        ],
        [
            s[4].v.y - s[0].v.y,
            s[0].v.x - s[4].v.x,
            s[0].p.y - s[4].p.y,
            s[4].p.x - s[0].p.x,
            s[4].p.x * s[4].v.y - s[4].p.y * s[4].v.x + s[0].p.y * s[0].v.x - s[0].p.x * s[0].v.y,
        ],
    ];

    // we can use a similar equation to solve for Pz (also solves Px/Vx, which we already have,
    // and Vz, which we don't need)
    let eqxz = [
        [
            s[1].v.z - s[0].v.z,
            s[0].v.x - s[1].v.x,
            s[0].p.z - s[1].p.z,
            s[1].p.x - s[0].p.x,
            s[1].p.x * s[1].v.z - s[1].p.z * s[1].v.x + s[0].p.z * s[0].v.x - s[0].p.x * s[0].v.z,
        ],
        [
            s[2].v.z - s[0].v.z,
            s[0].v.x - s[2].v.x,
            s[0].p.z - s[2].p.z,
            s[2].p.x - s[0].p.x,
            s[2].p.x * s[2].v.z - s[2].p.z * s[2].v.x + s[0].p.z * s[0].v.x - s[0].p.x * s[0].v.z,
        ],
        [
            s[3].v.z - s[0].v.z,
            s[0].v.x - s[3].v.x,
            s[0].p.z - s[3].p.z,
            s[3].p.x - s[0].p.x,
            s[3].p.x * s[3].v.z - s[3].p.z * s[3].v.x + s[0].p.z * s[0].v.x - s[0].p.x * s[0].v.z,
        ],
        [
            s[4].v.z - s[0].v.z,
            s[0].v.x - s[4].v.x,
            s[0].p.z - s[4].p.z,
            s[4].p.x - s[0].p.x,
            s[4].p.x * s[4].v.z - s[4].p.z * s[4].v.x + s[0].p.z * s[0].v.x - s[0].p.x * s[0].v.z,
        ],
    ];

    // none of these intermediate values fit into i64/f64, so we convert them to bignums and solve
    let v: Vec<_> = [eqxy, eqxz]
        .iter()
        .map(|eq| {
            eq.iter()
                .map(|r| r.iter().map(|c| c.to_bigint().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(linear_solve)
        .collect();

    // extract Px, Py, Pz, and sum for the answer
    (v[0][0].clone() + v[0][1].clone() + v[1][1].clone())
        .to_u64()
        .unwrap()
}

struct Stone {
    p: Vec3,
    v: Vec3,
}

struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Stone {
    fn parse(line: &str) -> Self {
        let s: Vec<_> = line.split(" @ ").collect();
        let p: Vec<_> = s[0]
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let v: Vec<_> = s[1]
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        Stone {
            p: Vec3 {
                x: p[0],
                y: p[1],
                z: p[2],
            },
            v: Vec3 {
                x: v[0],
                y: v[1],
                z: v[2],
            },
        }
    }
}

// a simple gaussian elimination solver
// bigint doesn't implement Copy, so we have to clone everywhere
fn linear_solve<T: Clone + num::Num>(mut eq: Vec<Vec<T>>) -> Vec<T> {
    // eliminate the lower triangle
    for j in 0..eq.len() - 1 {
        for i in j + 1..eq.len() {
            let f0 = eq[i][j].clone();
            let f1 = eq[j][j].clone();
            for k in 0..eq[0].len() {
                eq[i][k] = eq[i][k].clone() * f1.clone() - eq[j][k].clone() * f0.clone();
            }
        }
    }

    // eliminate the upper triangle
    for j in 1..eq.len() {
        for i in 0..j {
            let f0 = eq[i][j].clone();
            let f1 = eq[j][j].clone();
            for k in 0..eq[0].len() {
                eq[i][k] = eq[i][k].clone() * f1.clone() - eq[j][k].clone() * f0.clone();
            }
        }
    }

    // reduce the diagonal to 1
    for i in 0..eq.len() {
        let q = eq[i][i].clone();
        for k in 0..eq[0].len() {
            eq[i][k] = eq[i][k].clone() / q.clone();
        }
    }

    // extract the right column for the solutions
    eq.iter().map(|r| r[r.len() - 1].clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        19, 13, 30 @ -2, 1, -2\n\
        20, 25, 34 @ -2, -2, -4\n\
        18, 19, 22 @ -1, -1, -2\n\
        12, 31, 28 @ -1, -2, -1\n\
        20, 19, 15 @ 1, -5, -3\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA, 7, 27), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 47);
    }
}
