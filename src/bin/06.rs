use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Guard {
    pub position: Pos,
    pub direction: Direction,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(i32, i32);

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited: HashSet<Pos> = HashSet::new();

    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    let mut rocks: HashSet<Pos> = HashSet::new();
    let mut guard = Guard {
        position: Pos(0, 0),
        direction: Direction::Up,
    };

    for (y, row) in input.lines().enumerate() {
        x_max = row.len() as i32 - 1;
        y_max = y as i32;
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    rocks.insert(Pos(x as i32, y as i32));
                }
                '^' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Up,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                '>' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Right,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                '<' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Left,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                'v' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Down,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    while guard.position.0 >= 0
        && guard.position.0 <= x_max
        && guard.position.1 >= 0
        && guard.position.1 <= y_max
    {
        // Is forward rock
        let is_forward_rock = match guard.direction {
            Direction::Up => rocks.contains(&Pos(guard.position.0, guard.position.1 - 1)),
            Direction::Down => rocks.contains(&Pos(guard.position.0, guard.position.1 + 1)),
            Direction::Left => rocks.contains(&Pos(guard.position.0 - 1, guard.position.1)),
            Direction::Right => rocks.contains(&Pos(guard.position.0 + 1, guard.position.1)),
        };

        // Turn if needed
        if is_forward_rock {
            match guard.direction {
                Direction::Up => guard.direction = Direction::Right,
                Direction::Down => guard.direction = Direction::Left,
                Direction::Left => guard.direction = Direction::Up,
                Direction::Right => guard.direction = Direction::Down,
            }
        } else {
            let next_pos = match guard.direction {
                Direction::Up => Pos(guard.position.0, guard.position.1 - 1),
                Direction::Down => Pos(guard.position.0, guard.position.1 + 1),
                Direction::Left => Pos(guard.position.0 - 1, guard.position.1),
                Direction::Right => Pos(guard.position.0 + 1, guard.position.1),
            };
            visited.insert(next_pos);
            guard.position = next_pos;
        };
    }
    Some(visited.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut visited: HashSet<Pos> = HashSet::new();

    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    let mut rocks: HashSet<Pos> = HashSet::new();
    let mut guard = Guard {
        position: Pos(0, 0),
        direction: Direction::Up,
    };

    for (y, row) in input.lines().enumerate() {
        x_max = row.len() as i32 - 1;
        y_max = y as i32;
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    rocks.insert(Pos(x as i32, y as i32));
                }
                '^' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Up,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                '>' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Right,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                '<' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Left,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                'v' => {
                    guard = Guard {
                        position: Pos(x as i32, y as i32),
                        direction: Direction::Down,
                    };
                    visited.insert(Pos(x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    let start_guard = guard;

    let mut potential_blocker_positions = HashSet::new();
    while guard.position.0 >= 0
        && guard.position.0 <= x_max
        && guard.position.1 >= 0
        && guard.position.1 <= y_max
    {
        // Is forward rock
        let is_forward_rock = match guard.direction {
            Direction::Up => rocks.contains(&Pos(guard.position.0, guard.position.1 - 1)),
            Direction::Down => rocks.contains(&Pos(guard.position.0, guard.position.1 + 1)),
            Direction::Left => rocks.contains(&Pos(guard.position.0 - 1, guard.position.1)),
            Direction::Right => rocks.contains(&Pos(guard.position.0 + 1, guard.position.1)),
        };

        // Turn if needed
        if is_forward_rock {
            match guard.direction {
                Direction::Up => guard.direction = Direction::Right,
                Direction::Down => guard.direction = Direction::Left,
                Direction::Left => guard.direction = Direction::Up,
                Direction::Right => guard.direction = Direction::Down,
            }
        } else {
            let next_pos = match guard.direction {
                Direction::Up => Pos(guard.position.0, guard.position.1 - 1),
                Direction::Down => Pos(guard.position.0, guard.position.1 + 1),
                Direction::Left => Pos(guard.position.0 - 1, guard.position.1),
                Direction::Right => Pos(guard.position.0 + 1, guard.position.1),
            };
            potential_blocker_positions.insert(next_pos);
            guard.position = next_pos;
        };
    }

    let mut res = 0;
    for block in potential_blocker_positions {
        if check_for_loop(&rocks, x_max, y_max, block, start_guard) {
            res += 1
        };
    }

    Some(res)
}

fn walk_forward(rocks: &HashSet<Pos>, x_max: i32, y_max: i32, guard: Guard) -> (bool, Guard) {
    let mut new_guard = guard;

    loop {
        let new_pos = match new_guard.direction {
            Direction::Up => Pos(new_guard.position.0, new_guard.position.1 - 1),
            Direction::Down => Pos(new_guard.position.0, new_guard.position.1 + 1),
            Direction::Left => Pos(new_guard.position.0 - 1, new_guard.position.1),
            Direction::Right => Pos(new_guard.position.0 + 1, new_guard.position.1),
        };

        if new_pos.0 < 0 || new_pos.0 > x_max || new_pos.1 < 0 || new_pos.1 > y_max {
            return (true, new_guard);
        } else if match guard.direction {
            Direction::Up => rocks.contains(&new_pos),
            Direction::Down => rocks.contains(&new_pos),
            Direction::Left => rocks.contains(&new_pos),
            Direction::Right => rocks.contains(&new_pos),
        } {
            return (false, new_guard);
        };

        new_guard.position = new_pos;
    }
}

fn check_for_loop(
    rocks: &HashSet<Pos>,
    x_max: i32,
    y_max: i32,
    new_block: Pos,
    start_guard: Guard,
) -> bool {
    let mut guard = start_guard;
    let mut new_rocks = rocks.clone();
    new_rocks.insert(new_block);

    let mut visited = HashSet::new();
    loop {
        if visited.contains(&(guard.position, guard.direction)) {
            return true;
        }
        visited.insert((guard.position, guard.direction));
        let (exited, walked_guard) = walk_forward(&new_rocks, x_max, y_max, guard);

        if exited {
            return false;
        }

        guard.position = walked_guard.position;
        match guard.direction {
            Direction::Up => guard.direction = Direction::Right,
            Direction::Down => guard.direction = Direction::Left,
            Direction::Left => guard.direction = Direction::Up,
            Direction::Right => guard.direction = Direction::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
