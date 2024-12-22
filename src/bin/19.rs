use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (towels_str, wanted_str) = input.split_once("\n\n").unwrap();
    let towels = towels_str.split(",").map(|s| s.trim()).collect_vec();
    let wanted = wanted_str.lines().collect_vec();

    let mut results = vec![];

    let mut lookup: HashMap<String, bool> = HashMap::new();
    for w in wanted {
        let res = dfs(w, &towels, &mut lookup);
        if res {
            results.push(res);
        };
    }

    Some(results.len() as u32)
}

fn dfs(wanted: &str, towels: &[&str], lookup: &mut HashMap<String, bool>) -> bool {
    // Check Lookup
    if let Some(cached) = lookup.get(&wanted.to_string()) {
        return cached.to_owned();
    };

    let mut res = false;
    for t in towels {
        if let Some(stripped) = wanted.strip_prefix(t) {
            if stripped.is_empty() {
                res = true
            } else {
                let has_solution = dfs(stripped, towels, lookup);
                if has_solution {
                    res = true
                }
            }
        } else {
            continue;
        }
    }

    lookup.insert(wanted.to_owned(), res);
    res
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels_str, wanted_str) = input.split_once("\n\n").unwrap();
    let towels = towels_str.split(",").map(|s| s.trim()).collect_vec();
    let wanted = wanted_str.lines().collect_vec();

    let mut results: Vec<u64> = vec![];

    let mut lookup: HashMap<String, u64> = HashMap::new();
    for w in wanted {
        let res = dfs2(w, &towels, &mut lookup);
        results.push(res);
    }

    Some(results.iter().sum())
}

fn dfs2(wanted: &str, towels: &[&str], lookup: &mut HashMap<String, u64>) -> u64 {
    // Check Lookup
    if let Some(cached) = lookup.get(&wanted.to_string()) {
        return cached.to_owned();
    };

    let mut res = 0;
    for t in towels {
        if let Some(stripped) = wanted.strip_prefix(t) {
            if stripped.is_empty() {
                res += 1
            } else {
                res += dfs2(stripped, towels, lookup);
            }
        } else {
            continue;
        }
    }

    lookup.insert(wanted.to_owned(), res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
