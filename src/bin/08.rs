use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut antenna_groups = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        x_max = row.len() - 1;
        y_max = y;
        for (x, el) in row.chars().enumerate() {
            if el == '.' {
                continue;
            }
            antenna_groups
                .entry(el)
                .or_insert(vec![])
                .push(Position(x, y));
        }
    }

    let mut antinodes = HashSet::new();
    for antenna_positions in antenna_groups.values() {
        for position_pair in antenna_positions.iter().permutations(2) {
            if let Some(anode) = get_antinode(position_pair[0], position_pair[1], x_max, y_max) {
                antinodes.insert(anode);
            }
        }
    }

    Some(antinodes.len())
}

fn get_antinode(
    left: &Position,
    right: &Position,
    x_bound: usize,
    y_bound: usize,
) -> Option<Position> {
    // Left to Right distance
    let x_delta_lr: isize = right.0 as isize - left.0 as isize;
    let y_delta_lr: isize = right.1 as isize - left.1 as isize;
    let antinode_lr_x = right.0.checked_add_signed(x_delta_lr);
    let antinode_lr_y = right.1.checked_add_signed(y_delta_lr);
    if let (Some(x), Some(y)) = (antinode_lr_x, antinode_lr_y) {
        if x <= x_bound && y <= y_bound {
            return Some(Position(x, y));
        }
    };
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut antenna_groups = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        x_max = row.len() - 1;
        y_max = y;
        for (x, el) in row.chars().enumerate() {
            if el == '.' {
                continue;
            }
            antenna_groups
                .entry(el)
                .or_insert(vec![])
                .push(Position(x, y));
        }
    }

    let mut antinodes = HashSet::new();
    for antenna_positions in antenna_groups.values() {
        for position_pair in antenna_positions.iter().permutations(2) {
            let res = get_antinode_2(*position_pair[0], *position_pair[1], x_max, y_max);
            antinodes.extend(res);
        }
    }

    Some(antinodes.len())
}

fn get_antinode_2(
    left: Position,
    right: Position,
    x_bound: usize,
    y_bound: usize,
) -> Vec<Position> {
    let mut anodes = vec![left.to_owned(), right.to_owned()];
    let x_delta: isize = right.0 as isize - left.0 as isize;
    let y_delta: isize = right.1 as isize - left.1 as isize;
    let mut antinode_x = right.0.checked_add_signed(x_delta);
    let mut antinode_y = right.1.checked_add_signed(y_delta);
    while let (Some(x), Some(y)) = (antinode_x, antinode_y) {
        if x <= x_bound && y <= y_bound {
            anodes.push(Position(x, y));
            antinode_x = x.checked_add_signed(x_delta);
            antinode_y = y.checked_add_signed(y_delta);
        } else {
            break;
        }
    }

    anodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
