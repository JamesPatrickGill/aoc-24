use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let value: u32 = re
        .find_iter(input)
        .map(|c| {
            let mtch = c.as_str();
            return mtch[4..mtch.len() - 1]
                .split(",")
                .map(|a| a.parse::<u32>().unwrap())
                .product::<u32>();
        })
        .sum();
    Some(value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d*,\d*\)|do\(\)|don't\(\)").unwrap();
    let mut do_mul = true;
    let value: u32 = re
        .find_iter(input)
        .filter_map(|c| {
            let mtch = c.as_str();

            if mtch.contains("don't") {
                do_mul = false;
                return None;
            } else if mtch.contains("do") {
                do_mul = true;
                return None;
            }

            if do_mul {
                return Some(
                    mtch[4..mtch.len() - 1]
                        .split(",")
                        .map(|a| a.parse::<u32>().unwrap())
                        .product::<u32>(),
                );
            } else {
                None
            }
        })
        .sum();
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }
}
