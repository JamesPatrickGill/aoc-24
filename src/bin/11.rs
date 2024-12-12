use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lookup = HashMap::new();
    let stones: u64 = input
        .trim()
        .split(" ")
        .map(|num_str| {
            let curr_stone = num_str.parse().unwrap();
            blink(&mut lookup, curr_stone, 0)
        })
        .sum();
    Some(stones)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct LookupKey(u64, u8);

fn blink(lookup: &mut HashMap<LookupKey, u64>, curr_stone: u64, curr_step: u8) -> u64 {
    // Break case
    if curr_step == 25 {
        return 1;
    }

    // Check Lookup
    if let Some(&cached) = lookup.get(&LookupKey(curr_stone, curr_step)) {
        return cached;
    };

    let res = if curr_stone == 0 {
        blink(lookup, 1, curr_step + 1)
    } else if curr_stone.to_string().len() % 2 == 0 {
        let num_str = curr_stone.to_string();
        let (l, r) = num_str.split_at(num_str.len() / 2);
        blink(lookup, l.parse::<u64>().unwrap(), curr_step + 1)
            + blink(lookup, r.parse::<u64>().unwrap(), curr_step + 1)
    } else {
        blink(lookup, curr_stone * 2024, curr_step + 1)
    };

    lookup.insert(LookupKey(curr_stone, curr_step), res);

    res
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lookup = HashMap::new();
    let stones: u64 = input
        .trim()
        .split(" ")
        .map(|num_str| {
            let curr_stone = num_str.parse().unwrap();
            blink2(&mut lookup, curr_stone, 0)
        })
        .sum();
    Some(stones)
}

fn blink2(lookup: &mut HashMap<LookupKey, u64>, curr_stone: u64, curr_step: u8) -> u64 {
    // Break case
    if curr_step == 75 {
        return 1;
    }

    // Check Lookup
    if let Some(&cached) = lookup.get(&LookupKey(curr_stone, curr_step)) {
        return cached;
    };

    let res = if curr_stone == 0 {
        blink2(lookup, 1, curr_step + 1)
    } else if curr_stone.to_string().len() % 2 == 0 {
        let num_str = curr_stone.to_string();
        let (l, r) = num_str.split_at(num_str.len() / 2);
        blink2(lookup, l.parse::<u64>().unwrap(), curr_step + 1)
            + blink2(lookup, r.parse::<u64>().unwrap(), curr_step + 1)
    } else {
        blink2(lookup, curr_stone * 2024, curr_step + 1)
    };

    lookup.insert(LookupKey(curr_stone, curr_step), res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
