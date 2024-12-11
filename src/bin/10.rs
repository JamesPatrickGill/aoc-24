use std::{collections::HashSet, ops::Add};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let mut trailheads = vec![];

    let map = input
        .lines()
        .enumerate()
        .map(|(ydx, line)| {
            line.chars()
                .enumerate()
                .map(|(xdx, ch)| {
                    let num = ch.to_digit(10).unwrap_or(100);
                    if num == 0 {
                        trailheads.push((xdx, ydx));
                    }
                    num
                })
                .collect_vec()
        })
        .collect_vec();

    let mut res = 0;
    for th in trailheads {
        let tmp = dfs_step(&map, Position(th.0, th.1), 0);
        res += tmp.len();
    }

    Some(res)
}

fn dfs_step(map: &[Vec<u32>], current_pos: Position, current_val: u32) -> HashSet<Position> {
    if current_val == 9 {
        let mut set = HashSet::new();
        set.insert(current_pos);
        return set;
    }

    let mut result = HashSet::new();
    // Up
    if current_pos.1 > 0 && map[current_pos.1 - 1][current_pos.0] == current_val + 1 {
        result.extend(dfs_step(
            map,
            Position(current_pos.0, current_pos.1 - 1),
            current_val + 1,
        ));
    };

    // Righht
    if current_pos.0 < map[0].len() - 1 && map[current_pos.1][current_pos.0 + 1] == current_val + 1
    {
        result.extend(dfs_step(
            map,
            Position(current_pos.0 + 1, current_pos.1),
            current_val + 1,
        ));
    };

    // Down
    if current_pos.1 < map.len() - 1 && map[current_pos.1 + 1][current_pos.0] == current_val + 1 {
        result.extend(dfs_step(
            map,
            Position(current_pos.0, current_pos.1 + 1),
            current_val + 1,
        ));
    };

    // Left
    if current_pos.0 > 0 && map[current_pos.1][current_pos.0 - 1] == current_val + 1 {
        result.extend(dfs_step(
            map,
            Position(current_pos.0 - 1, current_pos.1),
            current_val + 1,
        ));
    };

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut trailheads = vec![];

    let map = input
        .lines()
        .enumerate()
        .map(|(ydx, line)| {
            line.chars()
                .enumerate()
                .map(|(xdx, ch)| {
                    let num = ch.to_digit(10).unwrap_or(100);
                    if num == 0 {
                        trailheads.push((xdx, ydx));
                    }
                    num
                })
                .collect_vec()
        })
        .collect_vec();

    let mut res = 0;
    for th in trailheads {
        res += dfs_step_2(&map, Position(th.0, th.1), 0);
    }

    Some(res)
}

fn dfs_step_2(map: &[Vec<u32>], current_pos: Position, current_val: u32) -> u32 {
    if current_val == 9 {
        return 1;
    }

    let mut result: u32 = 0;
    // Up
    if current_pos.1 > 0 && map[current_pos.1 - 1][current_pos.0] == current_val + 1 {
        result = result.add(dfs_step_2(
            map,
            Position(current_pos.0, current_pos.1 - 1),
            current_val + 1,
        ));
    };

    // Righht
    if current_pos.0 < map[0].len() - 1 && map[current_pos.1][current_pos.0 + 1] == current_val + 1
    {
        result = result.add(dfs_step_2(
            map,
            Position(current_pos.0 + 1, current_pos.1),
            current_val + 1,
        ));
    };

    // Down
    if current_pos.1 < map.len() - 1 && map[current_pos.1 + 1][current_pos.0] == current_val + 1 {
        result = result.add(dfs_step_2(
            map,
            Position(current_pos.0, current_pos.1 + 1),
            current_val + 1,
        ));
    };

    // Left
    if current_pos.0 > 0 && map[current_pos.1][current_pos.0 - 1] == current_val + 1 {
        result = result.add(dfs_step_2(
            map,
            Position(current_pos.0 - 1, current_pos.1),
            current_val + 1,
        ));
    };

    result
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
