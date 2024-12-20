use std::collections::{HashMap, HashSet};

advent_of_code::solution!(15);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(i32, i32);

fn sum_pos(pos1: Pos, pos2: Pos) -> Pos {
    Pos(pos1.0 + pos2.0, pos1.1 + pos2.1)
}

fn gps_coord(pos: Pos) -> i32 {
    (100 * pos.1) + pos.0
}

fn gps_coord2(pos: Pos) -> i32 {
    (100 * pos.1) + pos.0
}

pub fn part_one(input: &str) -> Option<i32> {
    let (map, directions) = input.split_once("\n\n").unwrap();
    let mut walls = HashSet::new();
    let mut rocks = HashSet::new();
    let mut guard = Pos(0, 0);
    for (y, row) in map.lines().enumerate() {
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    walls.insert(Pos(x as i32, y as i32));
                }
                'O' => {
                    rocks.insert(Pos(x as i32, y as i32));
                }
                '@' => guard = Pos(x as i32, y as i32),
                _ => {}
            }
        }
    }

    for dir in directions.trim().replace("\n", "").chars() {
        let delta_pos = match dir {
            '<' => Pos(-1, 0),
            '^' => Pos(0, -1),
            '>' => Pos(1, 0),
            'v' => Pos(0, 1),
            _ => panic!("Woah that's not a direction {}", dir),
        };

        let mut should_move = false;
        let mut rocks_to_move: HashSet<Pos> = HashSet::new();

        // Find what to move
        let mut next_candidate = sum_pos(guard, delta_pos);
        'l: loop {
            if walls.contains(&next_candidate) {
                break 'l;
            } else if rocks.contains(&next_candidate) {
                rocks_to_move.insert(next_candidate);
            } else {
                should_move = true;
                break 'l;
            }

            next_candidate = sum_pos(next_candidate, delta_pos)
        }

        // Move them
        if should_move {
            guard = sum_pos(guard, delta_pos);
            let mut new_rocks = HashSet::new();
            for rock in rocks_to_move {
                rocks.remove(&rock);
                new_rocks.insert(sum_pos(rock, delta_pos));
            }
            rocks.extend(new_rocks);
        }
    }

    let mut result = 0;
    for rock in rocks {
        result += gps_coord(rock)
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (map, directions) = input.split_once("\n\n").unwrap();
    let mut walls = HashSet::new();
    let mut rocks = HashSet::new();
    let mut rock_connections = HashMap::new();
    let mut guard = Pos(0, 0);
    for (y, row) in map.lines().enumerate() {
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    walls.insert(Pos(x as i32 * 2, y as i32));
                    walls.insert(Pos((x as i32 * 2) + 1, y as i32));
                }
                'O' => {
                    rocks.insert(Pos(x as i32 * 2, y as i32));
                    rocks.insert(Pos((x as i32 * 2) + 1, y as i32));
                    rock_connections.insert(
                        Pos(x as i32 * 2, y as i32),
                        Pos((x as i32 * 2) + 1, y as i32),
                    );
                    rock_connections.insert(
                        Pos((x as i32 * 2) + 1, y as i32),
                        Pos(x as i32 * 2, y as i32),
                    );
                }
                '@' => guard = Pos(x as i32 * 2, y as i32),
                _ => {}
            }
        }
    }

    for dir in directions.trim().replace("\n", "").chars() {
        let (delta_pos, is_vert) = match dir {
            '<' => (Pos(-1, 0), false),
            '^' => (Pos(0, -1), true),
            '>' => (Pos(1, 0), false),
            'v' => (Pos(0, 1), true),
            _ => panic!("Woah that's not a direction {}", dir),
        };

        let mut should_move = false;
        let mut rocks_to_move: HashSet<Pos> = HashSet::new();

        // Find what to move
        let mut move_frontier = HashSet::from_iter(vec![sum_pos(guard, delta_pos)]);
        'l: loop {
            let mut new_move_frontier = HashSet::new();
            let mut rocks_added = false;

            for movement in move_frontier {
                if walls.contains(&movement) {
                    break 'l;
                } else if rocks.contains(&movement) {
                    rocks_added = true;
                    if is_vert {
                        let connected_rock = rock_connections.get(&movement).unwrap().to_owned();

                        new_move_frontier.insert(sum_pos(movement, delta_pos));
                        new_move_frontier.insert(sum_pos(connected_rock, delta_pos));

                        rocks_to_move.insert(movement);
                        rocks_to_move.insert(connected_rock);
                    } else {
                        new_move_frontier.insert(sum_pos(movement, delta_pos));

                        rocks_to_move.insert(movement);
                    }
                } else {
                    new_move_frontier.insert(movement);
                };
            }

            if !rocks_added {
                should_move = true;
                break 'l;
            }

            move_frontier = new_move_frontier;
        }

        // Move them
        if should_move {
            guard = sum_pos(guard, delta_pos);

            let mut already_moved = HashSet::new();
            let mut new_rocks = HashSet::new();
            let mut new_connections = HashMap::new();

            for rock in rocks_to_move {
                if already_moved.contains(&rock) {
                    continue;
                }
                let connected_rock = rock_connections.get(&rock).unwrap().to_owned();

                rocks.remove(&rock);
                rocks.remove(&connected_rock);

                rock_connections.remove(&rock);
                rock_connections.remove(&connected_rock);

                already_moved.insert(rock);
                already_moved.insert(connected_rock);

                let new_connected_rock = sum_pos(connected_rock, delta_pos);
                let new_rock = sum_pos(rock, delta_pos);

                new_rocks.insert(new_rock);
                new_rocks.insert(new_connected_rock);

                new_connections.insert(new_rock, new_connected_rock);
                new_connections.insert(new_connected_rock, new_rock);
            }

            rock_connections.extend(new_connections);
            rocks.extend(new_rocks);
        }
    }

    let mut sig_rocks = HashSet::new();
    let mut processed_rocks = HashSet::new();
    for rock in rocks {
        if processed_rocks.contains(&rock) {
            continue;
        }
        let connected_rock = rock_connections.get(&rock).unwrap().to_owned();

        let l_rock = if rock.0 < connected_rock.0 {
            rock
        } else {
            connected_rock
        };
        sig_rocks.insert(l_rock);

        processed_rocks.insert(rock);
        processed_rocks.insert(connected_rock);
    }

    let mut result = 0;
    for rock in sig_rocks {
        result += gps_coord2(rock)
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
