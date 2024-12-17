use std::ops::Rem;

use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(i64, i64);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = input
        .split("\n\n")
        .map(|maching_def| {
            let lines = maching_def.trim().lines().collect_vec();

            let ax_idx = lines[0].find("X+").unwrap();
            let ax: i64 = lines[0][ax_idx + 2..ax_idx + 4].parse().unwrap();
            let ay_idx = lines[0].find("Y+").unwrap();
            let ay: i64 = lines[0][ay_idx + 2..ay_idx + 4].parse().unwrap();

            let bx_idx = lines[1].find("X+").unwrap();
            let bx: i64 = lines[1][bx_idx + 2..bx_idx + 4].parse().unwrap();
            let by_idx = lines[1].find("Y+").unwrap();
            let by: i64 = lines[1][by_idx + 2..by_idx + 4].parse().unwrap();

            let px_idx = lines[2].find("X=").unwrap();
            let pxc_idx = lines[2].find(",").unwrap();
            let px: i64 = lines[2][px_idx + 2..pxc_idx].parse().unwrap();
            let py_idx = lines[2].find("Y=").unwrap();
            let py: i64 = lines[2][py_idx + 2..].parse().unwrap();

            Machine {
                a: Pos(ax, ay),
                b: Pos(bx, by),
                prize: Pos(px, py),
            }
        })
        .collect_vec();

    let mut res = 0;
    for mach in machines {
        if let Some(sol) = solve(mach) {
            res += sol
        };
    }

    Some(res)
}

fn solve(mach: Machine) -> Option<i64> {
    let determinant = mach.a.0 * mach.b.1 - mach.a.1 * mach.b.0;
    if determinant == 0 {
        return None;
    }

    if (mach.b.1 * mach.prize.0 - mach.b.0 * mach.prize.1).rem(determinant) != 0
        || (mach.a.0 * mach.prize.1 - mach.a.1 * mach.prize.0).rem(determinant) != 0
    {
        return None;
    }

    let a_clicks = (mach.b.1 * mach.prize.0 - mach.b.0 * mach.prize.1).checked_div(determinant);
    let b_clicks = (mach.a.0 * mach.prize.1 - mach.a.1 * mach.prize.0).checked_div(determinant);

    if let (Some(a), Some(b)) = (a_clicks, b_clicks) {
        Some(a * 3 + b)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = input
        .split("\n\n")
        .map(|maching_def| {
            let lines = maching_def.trim().lines().collect_vec();

            let ax_idx = lines[0].find("X+").unwrap();
            let ax: i64 = lines[0][ax_idx + 2..ax_idx + 4].parse().unwrap();
            let ay_idx = lines[0].find("Y+").unwrap();
            let ay: i64 = lines[0][ay_idx + 2..ay_idx + 4].parse().unwrap();

            let bx_idx = lines[1].find("X+").unwrap();
            let bx: i64 = lines[1][bx_idx + 2..bx_idx + 4].parse().unwrap();
            let by_idx = lines[1].find("Y+").unwrap();
            let by: i64 = lines[1][by_idx + 2..by_idx + 4].parse().unwrap();

            let px_idx = lines[2].find("X=").unwrap();
            let pxc_idx = lines[2].find(",").unwrap();
            let px: i64 = lines[2][px_idx + 2..pxc_idx].parse().unwrap();
            let py_idx = lines[2].find("Y=").unwrap();
            let py: i64 = lines[2][py_idx + 2..].parse().unwrap();

            Machine {
                a: Pos(ax, ay),
                b: Pos(bx, by),
                prize: Pos(px + 10000000000000, py + 10000000000000),
            }
        })
        .collect_vec();

    let mut res = 0;
    for mach in machines {
        if let Some(sol) = solve(mach) {
            res += sol
        };
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
