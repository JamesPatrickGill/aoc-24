use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|a| a.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut bad_reports = 0;
    for report in reports.iter() {
        let mut dir = 0;
        for idx in 0..report.len() - 1 {
            let l = report.get(idx).unwrap();
            let r = report.get(idx + 1).unwrap();
            match r.cmp(l) {
                std::cmp::Ordering::Less => {
                    if dir == 1 || l - r > 3 {
                        bad_reports += 1;
                        break;
                    }
                    dir = -1;
                }
                std::cmp::Ordering::Equal => {
                    bad_reports += 1;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    if dir == -1 || r - l > 3 {
                        bad_reports += 1;
                        break;
                    }
                    dir = 1;
                }
            }
        }
    }
    Some((reports.len() - bad_reports) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|a| a.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut good_reports = 0;
    for report in reports.iter() {
        let mut to_check_queue = VecDeque::new();
        to_check_queue.push_back(report);
        while !to_check_queue.is_empty() {
            if is_safe(report) {
                good_reports += 1
            } else {
                for idx in 0..report.len() {
                    let mut new_report = report.clone();
                    new_report.remove(idx);
                    if is_safe(&new_report) {
                        good_reports += 1;
                        break;
                    }
                }
            };

            to_check_queue.pop_front();
        }
    }

    Some(good_reports)
}

fn is_safe(report: &[i32]) -> bool {
    let mut dir = 0;
    for idx in 0..report.len() - 1 {
        let l = report.get(idx).unwrap();
        let r = report.get(idx + 1).unwrap();
        match r.cmp(l) {
            std::cmp::Ordering::Less => {
                if dir == 1 || l - r > 3 {
                    return false;
                }
                dir = -1;
            }
            std::cmp::Ordering::Equal => {
                return false;
            }
            std::cmp::Ordering::Greater => {
                if dir == -1 || r - l > 3 {
                    return false;
                }
                dir = 1;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
