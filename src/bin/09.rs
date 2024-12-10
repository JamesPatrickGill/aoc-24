use std::{collections::VecDeque, ops::Div};

advent_of_code::solution!(9);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Block {
    File(u32),
    Free(u32),
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut queue: VecDeque<Block> =
        VecDeque::from_iter(input.chars().enumerate().filter_map(|(idx, ch)| {
            if let Some(value) = ch.to_digit(10) {
                if idx % 2 == 0 {
                    Some(Block::File(value))
                } else {
                    Some(Block::Free(value))
                }
            } else {
                None
            }
        }));
    let mut curr_lower_file_id: u64 = 0;
    let mut curr_upper_file_id: u64 = (queue.len() - 2).div_ceil(2) as u64;
    let mut curr_idx = 0;

    let mut result: u64 = 0;
    while !queue.is_empty() {
        let Some(front_candidate) = queue.pop_front() else {
            break;
        };
        match front_candidate {
            Block::File(size) => {
                for offset in 0..size {
                    result += curr_lower_file_id * (curr_idx + offset) as u64;
                }
                curr_idx += size;
                curr_lower_file_id += 1;
            }
            Block::Free(size) => {
                let r_size = if let Some(r) = queue.pop_back() {
                    match r {
                        Block::File(r_size) => r_size,
                        Block::Free(_) => {
                            if let Some(r2) = queue.pop_back() {
                                match r2 {
                                    Block::File(r2_size) => r2_size,
                                    Block::Free(_) => panic!("Double free lol"),
                                }
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    break;
                };

                match size.cmp(&r_size) {
                    std::cmp::Ordering::Less => {
                        for offset in 0..size {
                            result += curr_upper_file_id * (curr_idx + offset) as u64;
                        }
                        queue.push_back(Block::File(r_size - size));
                        curr_idx += size;
                    }
                    std::cmp::Ordering::Equal => {
                        for offset in 0..size {
                            result += curr_upper_file_id * (curr_idx + offset) as u64;
                        }
                        curr_upper_file_id -= 1;
                        curr_idx += size;
                    }
                    std::cmp::Ordering::Greater => {
                        for offset in 0..r_size {
                            result += curr_upper_file_id * (curr_idx + offset) as u64;
                        }
                        queue.push_front(Block::Free(size - r_size));
                        curr_upper_file_id -= 1;
                        curr_idx += r_size;
                    }
                }
            }
        }
    }

    Some(result)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Block2 {
    File(u32, u32),
    Free(u32),
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut queue: VecDeque<Block2> =
        VecDeque::from_iter(input.chars().enumerate().filter_map(|(idx, ch)| {
            if let Some(value) = ch.to_digit(10) {
                if idx % 2 == 0 {
                    Some(Block2::File(value, (idx as u32).div(2)))
                } else {
                    Some(Block2::Free(value))
                }
            } else {
                None
            }
        }));
    let mut curr_idx = 0;

    let mut result: u64 = 0;
    while !queue.is_empty() {
        let Some(front_candidate) = queue.pop_front() else {
            break;
        };
        match front_candidate {
            Block2::File(size, id) => {
                for offset in 0..size {
                    result += id as u64 * (curr_idx + offset) as u64;
                }
                curr_idx += size;
            }
            Block2::Free(size) => {
                // First get next right side candidate that fits
                let mut to_readd_to_queue = VecDeque::new();
                let mut r_candidate = None;
                loop {
                    if queue.is_empty() {
                        queue.extend(to_readd_to_queue);
                        break;
                    }

                    if let Some(r) = queue.pop_back() {
                        match r {
                            Block2::File(r_size, id) => match size.cmp(&r_size) {
                                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                                    r_candidate = Some((r_size, id));
                                    to_readd_to_queue.push_front(Block2::Free(r_size));
                                    queue.extend(to_readd_to_queue);
                                    break;
                                }
                                std::cmp::Ordering::Less => {
                                    to_readd_to_queue.push_front(Block2::File(r_size, id));
                                }
                            },
                            Block2::Free(_) => {
                                to_readd_to_queue.push_front(r);
                            }
                        }
                    } else {
                        queue.extend(to_readd_to_queue);
                        break;
                    };
                }

                // Now fit it if it can
                if let Some(r_can) = r_candidate {
                    match size.cmp(&r_can.0) {
                        std::cmp::Ordering::Less => panic!("Should not be possible"),
                        std::cmp::Ordering::Equal => {
                            for offset in 0..r_can.0 {
                                result += r_can.1 as u64 * (curr_idx + offset) as u64;
                            }
                            curr_idx += r_can.0;
                        }
                        std::cmp::Ordering::Greater => {
                            for offset in 0..r_can.0 {
                                result += r_can.1 as u64 * (curr_idx + offset) as u64;
                            }
                            curr_idx += r_can.0;
                            queue.push_front(Block2::Free(size - r_can.0));
                        }
                    }
                } else {
                    curr_idx += size;
                }
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
