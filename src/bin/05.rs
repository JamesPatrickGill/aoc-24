use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rule_lookup =
        rules
            .lines()
            .fold(HashMap::new(), |mut acc: HashMap<&str, Vec<&str>>, line| {
                let (l, r) = line.split_once("|").unwrap();
                acc.entry(l).or_default().push(r);
                acc
            });

    let mut result = 0;
    for update in updates.lines() {
        let mut already_seen: HashSet<&str> = HashSet::new();
        let mut success = true;
        for value in update.split(',') {
            let Some(must_come_after) = rule_lookup.get(value) else {
                already_seen.insert(value);
                continue;
            };
            let breaks_rule = must_come_after.iter().any(|val| already_seen.contains(val));
            if breaks_rule {
                success = false;
                break;
            }

            already_seen.insert(value);
        }

        if success {
            let as_strs = update.split(",").collect_vec();
            let middle = as_strs[(as_strs.len() - 1) / 2].parse::<u32>().unwrap();
            result += middle;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rule_lookup =
        rules
            .lines()
            .fold(HashMap::new(), |mut acc: HashMap<&str, Vec<&str>>, line| {
                let (l, r) = line.split_once("|").unwrap();
                acc.entry(l).or_default().push(r);
                acc
            });

    let mut incorrect = vec![];
    for update in updates.lines() {
        let mut already_seen: HashSet<&str> = HashSet::new();
        for value in update.split(',') {
            let Some(must_come_after) = rule_lookup.get(value) else {
                already_seen.insert(value);
                continue;
            };
            let breaks_rule = must_come_after.iter().any(|val| already_seen.contains(val));
            if breaks_rule {
                incorrect.push(update);
                break;
            }

            already_seen.insert(value);
        }
    }

    let mut res = 0;
    for update in incorrect.into_iter() {
        let ordered = update
            .split(",")
            .sorted_by(|a, b| {
                if !rule_lookup.contains_key(a) {
                    return Ordering::Greater;
                }

                if rule_lookup[a].contains(b) {
                    return Ordering::Less;
                }

                Ordering::Greater
            })
            .collect_vec();

        let middle = ordered[(ordered.len() - 1) / 2].parse::<u32>().unwrap();
        res += middle;
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
