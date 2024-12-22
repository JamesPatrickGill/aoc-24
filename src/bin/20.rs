use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::yield_now,
    vec,
};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(i32, i32);

fn sum_pos(pos1: Pos, pos2: Pos) -> Pos {
    Pos(pos1.0 + pos2.0, pos1.1 + pos2.1)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Move {
    pos: Pos,
    score: u32,
    has_cheated: Option<Pos>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut walls = HashSet::new();
    let mut racer = Pos(0, 0);
    let mut end = Pos(0, 0);
    let mut x_max = 0i32;
    let mut y_max = 0i32;
    for (y, row) in input.lines().enumerate() {
        y_max = y as i32;
        x_max = row.len() as i32 - 1;
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    walls.insert(Pos(x as i32, y as i32));
                }
                'S' => {
                    racer = Pos(x as i32, y as i32);
                }
                'E' => {
                    end = Pos(x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    // Find no cheat path
    let mut no_cheat_path = vec![racer];
    let mut no_cheat_lookup = HashMap::from([(racer, 0)]);

    let mut curr = racer;
    loop {
        if curr == end {
            break;
        }

        // Find next
        let possibilities = [
            (sum_pos(curr, Pos(0, -1))),
            (sum_pos(curr, Pos(1, 0))),
            (sum_pos(curr, Pos(0, 1))),
            (sum_pos(curr, Pos(-1, 0))),
        ];

        let mut result = Pos(0, 0);
        for possible_next in possibilities {
            if walls.contains(&possible_next) || no_cheat_lookup.contains_key(&possible_next) {
                continue;
            }

            result = possible_next;
        }

        no_cheat_path.push(result);
        no_cheat_lookup.insert(result, no_cheat_path.len() - 1);
        curr = result;
    }

    // let no_cheat_score = no_cheat_lookup.get(&end).unwrap();

    // walk path and look for cheats
    let mut all_saves = vec![];
    for step in no_cheat_path {
        all_saves.extend(look_for_cheats(
            step,
            &walls,
            &no_cheat_lookup,
            x_max,
            y_max,
        ));
    }

    let res = all_saves.into_iter().filter(|&el| el >= 100).count();

    Some(res)
}

fn look_for_cheats(
    step: Pos,
    walls: &HashSet<Pos>,
    lookup: &HashMap<Pos, usize>,
    x_max: i32,
    y_max: i32,
) -> Vec<usize> {
    // Find next
    let possibilities = [
        (sum_pos(step, Pos(0, -2))),
        (sum_pos(step, Pos(1, -1))),
        (sum_pos(step, Pos(2, 0))),
        (sum_pos(step, Pos(1, 1))),
        (sum_pos(step, Pos(0, 2))),
        (sum_pos(step, Pos(-1, 1))),
        (sum_pos(step, Pos(-2, 0))),
        (sum_pos(step, Pos(-1, -1))),
    ];

    let mut saved_times = vec![];
    for p in possibilities {
        if walls.contains(&p) || p.0 < 0 || p.1 < 0 || p.0 > x_max || p.1 > y_max {
            continue;
        }

        let no_cheat_score = lookup.get(&p).unwrap();
        let cheat_score = lookup.get(&step).unwrap() + 2;

        if &cheat_score < no_cheat_score {
            saved_times.push(no_cheat_score - cheat_score);
        }
    }

    saved_times
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut walls = HashSet::new();
    let mut racer = Pos(0, 0);
    let mut end = Pos(0, 0);
    let mut x_max = 0i32;
    let mut y_max = 0i32;
    for (y, row) in input.lines().enumerate() {
        y_max = y as i32;
        x_max = row.len() as i32 - 1;
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    walls.insert(Pos(x as i32, y as i32));
                }
                'S' => {
                    racer = Pos(x as i32, y as i32);
                }
                'E' => {
                    end = Pos(x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    // Find no cheat path
    let mut no_cheat_path = vec![racer];
    let mut no_cheat_lookup = HashMap::from([(racer, 0)]);

    let mut curr = racer;
    loop {
        if curr == end {
            break;
        }

        // Find next
        let possibilities = [
            (sum_pos(curr, Pos(0, -1))),
            (sum_pos(curr, Pos(1, 0))),
            (sum_pos(curr, Pos(0, 1))),
            (sum_pos(curr, Pos(-1, 0))),
        ];

        let mut result = Pos(0, 0);
        for possible_next in possibilities {
            if walls.contains(&possible_next) || no_cheat_lookup.contains_key(&possible_next) {
                continue;
            }

            result = possible_next;
        }

        no_cheat_path.push(result);
        no_cheat_lookup.insert(result, no_cheat_path.len() - 1);
        curr = result;
    }

    // let no_cheat_score = no_cheat_lookup.get(&end).unwrap();

    // walk path and look for cheats
    let mut all_saves = vec![];
    for step in no_cheat_path {
        all_saves.extend(look_for_cheats2(
            step,
            &walls,
            &no_cheat_lookup,
            x_max,
            y_max,
        ));
    }

    all_saves.sort();
    let res = all_saves.into_iter().filter(|&el| el >= 100).count();

    Some(res)
}

fn look_for_cheats2(
    step: Pos,
    walls: &HashSet<Pos>,
    lookup: &HashMap<Pos, usize>,
    x_max: i32,
    y_max: i32,
) -> Vec<usize> {
    // Find next
    let mut possibilities = HashSet::new();
    for x in 0..21 {
        for y in 0..21 {
            if x + y <= 20 {
                possibilities.insert(Pos(x, y));
                possibilities.insert(Pos(-x, y));
                possibilities.insert(Pos(x, -y));
                possibilities.insert(Pos(-x, -y));
            }
        }
    }

    let mut saved_times = vec![];
    for poss in possibilities {
        let p = sum_pos(step, poss);

        if walls.contains(&p)
            || p.0 < 0
            || p.1 < 0
            || p.0 > x_max
            || p.1 > y_max
            || lookup.get(&p).is_none()
        {
            continue;
        }

        let no_cheat_score = lookup.get(&p).unwrap();
        let cheat_score = lookup.get(&step).unwrap()
            + poss.0.unsigned_abs() as usize
            + poss.1.unsigned_abs() as usize;

        if &cheat_score < no_cheat_score {
            saved_times.push(no_cheat_score - cheat_score);
        }
    }

    saved_times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
