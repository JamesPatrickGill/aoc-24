use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let (left_int, right_int): (u32, u32) = line
            .split_once("   ")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();
        left.push(left_int);
        right.push(right_int);
    }
    left.sort_unstable();
    right.sort_unstable();

    let mut diff = 0;
    for idx in 0..left.len() {
        let l_el = left.get(idx).unwrap();
        let r_el = right.get(idx).unwrap();

        if l_el > r_el {
            diff += l_el - r_el;
        } else {
            diff += r_el - l_el;
        }
    }
    Some(diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = HashMap::new();
    for line in input.lines() {
        let (left_int, right_int): (u32, u32) = line
            .split_once("   ")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();
        left.push(left_int);
        *right.entry(right_int).or_insert(0) += 1;
    }
    left.sort_unstable();

    let mut diff = 0;
    for idx in 0..left.len() {
        let l_el = left.get(idx).unwrap();
        let r_el = right.get(l_el).unwrap_or(&0);

        diff += l_el * r_el;
    }
    Some(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
