use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(usize, usize);

#[derive(Debug, Clone)]
struct Region {
    pub perimeter: u64,
    pub positions: HashSet<Pos>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut already_processed = HashSet::new();

    let mut result = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if already_processed.contains(&Pos(x, y)) {
                continue;
            }

            let region = collect_region(&map, Pos(x, y), ch);
            result += region.perimeter * region.positions.len() as u64;
            already_processed.extend(region.positions);
        }
    }

    Some(result)
}

fn collect_region(map: &[Vec<char>], start: Pos, label: char) -> Region {
    let mut region = Region {
        perimeter: 0,
        positions: HashSet::new(),
    };

    let mut queue = VecDeque::from_iter(vec![start]);

    while let Some(candidate) = queue.pop_front() {
        if region.positions.contains(&candidate) {
            continue;
        }
        region.positions.insert(candidate);

        // Look up
        if candidate.1 > 0 && map[candidate.1 - 1][candidate.0] == label {
            let pos = Pos(candidate.0, candidate.1 - 1);
            queue.push_back(pos);
        } else {
            region.perimeter += 1
        };

        // Look right
        if candidate.0 < map[0].len() - 1 && map[candidate.1][candidate.0 + 1] == label {
            let pos = Pos(candidate.0 + 1, candidate.1);
            queue.push_back(pos);
        } else {
            region.perimeter += 1
        };

        // Look down
        if candidate.1 < map.len() - 1 && map[candidate.1 + 1][candidate.0] == label {
            let pos = Pos(candidate.0, candidate.1 + 1);
            queue.push_back(pos);
        } else {
            region.perimeter += 1
        };

        // Look left
        if candidate.0 > 0 && map[candidate.1][candidate.0 - 1] == label {
            let pos = Pos(candidate.0 - 1, candidate.1);
            queue.push_back(pos);
        } else {
            region.perimeter += 1
        };
    }

    region
}

#[derive(Debug, Clone)]
struct Region2 {
    pub edges: HashMap<Direction, HashSet<Pos>>,
    pub positions: HashSet<Pos>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut already_processed = HashSet::new();
    let mut result = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if already_processed.contains(&Pos(x, y)) {
                continue;
            }

            let region = collect_region_2(&map, Pos(x, y), ch);
            already_processed.extend(region.positions.clone());

            result += calculate_cost(region, map.len() - 1, map[0].len() - 1);
        }
    }

    Some(result)
}

fn collect_region_2(map: &[Vec<char>], start: Pos, label: char) -> Region2 {
    let mut region = Region2 {
        edges: HashMap::new(),
        positions: HashSet::new(),
    };

    let mut queue = VecDeque::from_iter(vec![start]);

    while let Some(candidate) = queue.pop_front() {
        if region.positions.contains(&candidate) {
            continue;
        }
        region.positions.insert(candidate);

        // Look up
        if candidate.1 > 0 && map[candidate.1 - 1][candidate.0] == label {
            let pos = Pos(candidate.0, candidate.1 - 1);
            queue.push_back(pos);
        } else {
            region
                .edges
                .entry(Direction::U)
                .or_default()
                .insert(candidate);
        };

        // Look right
        if candidate.0 < map[0].len() - 1 && map[candidate.1][candidate.0 + 1] == label {
            let pos = Pos(candidate.0 + 1, candidate.1);
            queue.push_back(pos);
        } else {
            region
                .edges
                .entry(Direction::R)
                .or_default()
                .insert(candidate);
        };

        // Look down
        if candidate.1 < map.len() - 1 && map[candidate.1 + 1][candidate.0] == label {
            let pos = Pos(candidate.0, candidate.1 + 1);
            queue.push_back(pos);
        } else {
            region
                .edges
                .entry(Direction::D)
                .or_default()
                .insert(candidate);
        };

        // Look left
        if candidate.0 > 0 && map[candidate.1][candidate.0 - 1] == label {
            let pos = Pos(candidate.0 - 1, candidate.1);
            queue.push_back(pos);
        } else {
            region
                .edges
                .entry(Direction::L)
                .or_default()
                .insert(candidate);
        };
    }

    region
}

fn calculate_cost(region: Region2, x_max: usize, y_max: usize) -> u64 {
    let mut count = 0;

    let mut visited_u = HashSet::new();
    let up_edges = region
        .edges
        .get(&Direction::U)
        .unwrap_or(&HashSet::new())
        .to_owned();
    for pos in up_edges.iter() {
        if visited_u.contains(pos) {
            continue;
        }
        visited_u.insert(pos.to_owned());

        // Look left
        let mut l_cand = pos.to_owned();
        loop {
            if l_cand.0 == 0 {
                break;
            };

            let poss_l_cand = Pos(l_cand.0 - 1, l_cand.1);
            if !up_edges.contains(&poss_l_cand) {
                break;
            }

            visited_u.insert(poss_l_cand);
            l_cand = poss_l_cand;
        }

        // Look right
        let mut r_cand = pos.to_owned();
        loop {
            if r_cand.0 > x_max - 1 {
                break;
            };

            let poss_l_cand = Pos(r_cand.0 + 1, r_cand.1);
            if !up_edges.contains(&poss_l_cand) {
                break;
            }

            visited_u.insert(poss_l_cand);
            r_cand = poss_l_cand;
        }

        count += 1
    }

    let mut visited_d = HashSet::new();
    let down_edges = region
        .edges
        .get(&Direction::D)
        .unwrap_or(&HashSet::new())
        .to_owned();
    for pos in down_edges.iter() {
        if visited_d.contains(pos) {
            continue;
        }
        visited_d.insert(pos.to_owned());

        // Look left
        let mut l_cand = pos.to_owned();
        loop {
            if l_cand.0 == 0 {
                break;
            };

            let poss_l_cand = Pos(l_cand.0 - 1, l_cand.1);
            if !down_edges.contains(&poss_l_cand) {
                break;
            }

            visited_d.insert(poss_l_cand);
            l_cand = poss_l_cand;
        }

        // Look right
        let mut r_cand = pos.to_owned();
        loop {
            if r_cand.0 > x_max - 1 {
                break;
            };

            let poss_l_cand = Pos(r_cand.0 + 1, r_cand.1);
            if !down_edges.contains(&poss_l_cand) {
                break;
            }

            visited_d.insert(poss_l_cand);
            r_cand = poss_l_cand;
        }

        count += 1
    }

    let mut visited_l = HashSet::new();
    let left_edges = region
        .edges
        .get(&Direction::L)
        .unwrap_or(&HashSet::new())
        .to_owned();
    for pos in left_edges.iter() {
        if visited_l.contains(pos) {
            continue;
        }
        visited_l.insert(pos.to_owned());

        // Look up
        let mut u_cand = pos.to_owned();
        loop {
            if u_cand.1 == 0 {
                break;
            };

            let poss_u_cand = Pos(u_cand.0, u_cand.1 - 1);
            if !left_edges.contains(&poss_u_cand) {
                break;
            }

            visited_l.insert(poss_u_cand);
            u_cand = poss_u_cand;
        }

        // Look down
        let mut d_cand = pos.to_owned();
        loop {
            if d_cand.1 > y_max - 1 {
                break;
            };

            let poss_d_cand = Pos(d_cand.0, d_cand.1 + 1);
            if !left_edges.contains(&poss_d_cand) {
                break;
            }

            visited_l.insert(poss_d_cand);
            d_cand = poss_d_cand;
        }

        count += 1
    }

    let mut visited_r = HashSet::new();
    let right_edges = region
        .edges
        .get(&Direction::R)
        .unwrap_or(&HashSet::new())
        .to_owned();
    for pos in right_edges.iter() {
        if visited_r.contains(pos) {
            continue;
        }
        visited_r.insert(pos.to_owned());

        // Look up
        let mut u_cand = pos.to_owned();
        loop {
            if u_cand.1 == 0 {
                break;
            };

            let poss_u_cand = Pos(u_cand.0, u_cand.1 - 1);
            if !right_edges.contains(&poss_u_cand) {
                break;
            }

            visited_r.insert(poss_u_cand);
            u_cand = poss_u_cand;
        }

        // Look down
        let mut d_cand = pos.to_owned();
        loop {
            if d_cand.1 > y_max - 1 {
                break;
            };

            let poss_d_cand = Pos(d_cand.0, d_cand.1 + 1);
            if !right_edges.contains(&poss_d_cand) {
                break;
            }

            visited_r.insert(poss_d_cand);
            d_cand = poss_d_cand;
        }

        count += 1
    }

    count * region.positions.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(692));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(236));
    }
}
