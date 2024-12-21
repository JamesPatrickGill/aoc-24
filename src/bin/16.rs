use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(i32, i32);

fn sum_pos(pos1: Pos, pos2: Pos) -> Pos {
    Pos(pos1.0 + pos2.0, pos1.1 + pos2.1)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Move {
    pos: Pos,
    dir: Dir,
    score: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut walls = HashSet::new();
    let mut reindeer_start = Pos(0, 0);
    let mut end = Pos(0, 0);
    for (y, row) in input.lines().enumerate() {
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    walls.insert(Pos(x as i32, y as i32));
                }
                'S' => {
                    reindeer_start = Pos(x as i32, y as i32);
                }
                'E' => {
                    end = Pos(x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    let mut min = 1000000;

    let mut visited: HashMap<(Pos, Dir), u32> = HashMap::new();
    let mut queue = VecDeque::from_iter(vec![Move {
        pos: reindeer_start,
        dir: Dir::E,
        score: 0,
    }]);

    while let Some(cand) = queue.pop_front() {
        // handle end case
        if cand.pos == end && cand.score < min {
            min = cand.score;
            continue;
        }

        // handle already seen but better
        if let Some(cached) = visited.get(&(cand.pos, cand.dir)) {
            if cached < &cand.score {
                continue;
            }
        };
        *visited.entry((cand.pos, cand.dir)).or_default() = cand.score;

        let next_pos_and_dirs = vec![
            (sum_pos(cand.pos, Pos(0, -1)), Dir::N),
            (sum_pos(cand.pos, Pos(1, 0)), Dir::E),
            (sum_pos(cand.pos, Pos(0, 1)), Dir::S),
            (sum_pos(cand.pos, Pos(-1, 0)), Dir::W),
        ];

        for (p, d) in next_pos_and_dirs {
            if walls.contains(&p) {
                continue;
            }

            if cand.dir == d {
                queue.push_back(Move {
                    pos: p,
                    dir: d,
                    score: cand.score + 1,
                })
            } else {
                queue.push_back(Move {
                    pos: p,
                    dir: d,
                    score: cand.score + 1001,
                })
            }
        }
    }

    Some(min)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct MoveWithPath {
    pos: Pos,
    dir: Dir,
    score: u32,
    path: HashSet<Pos>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut walls = HashSet::new();
    let mut reindeer_start = Pos(0, 0);
    let mut end = Pos(0, 0);
    for (y, row) in input.lines().enumerate() {
        for (x, el) in row.chars().enumerate() {
            match el {
                '#' => {
                    walls.insert(Pos(x as i32, y as i32));
                }
                'S' => {
                    reindeer_start = Pos(x as i32, y as i32);
                }
                'E' => {
                    end = Pos(x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    let mut min = 1000000;
    let mut tiles = HashSet::new();

    let mut visited: HashMap<(Pos, Dir), u32> = HashMap::new();
    let mut queue = VecDeque::from_iter(vec![MoveWithPath {
        pos: reindeer_start,
        dir: Dir::E,
        score: 0,
        path: HashSet::from_iter(vec![reindeer_start]),
    }]);

    while let Some(cand) = queue.pop_front() {
        // handle end case
        if cand.pos == end {
            match cand.score.cmp(&min) {
                std::cmp::Ordering::Less => {
                    min = cand.score;
                    tiles = cand.path;
                }
                std::cmp::Ordering::Equal => {
                    tiles.extend(cand.path);
                }
                std::cmp::Ordering::Greater => {}
            }
            continue;
        }

        // handle already seen but better
        if let Some(cached) = visited.get(&(cand.pos, cand.dir)) {
            if cached < &cand.score {
                continue;
            }
        };
        *visited.entry((cand.pos, cand.dir)).or_default() = cand.score;

        let next_pos_and_dirs = vec![
            (sum_pos(cand.pos, Pos(0, -1)), Dir::N),
            (sum_pos(cand.pos, Pos(1, 0)), Dir::E),
            (sum_pos(cand.pos, Pos(0, 1)), Dir::S),
            (sum_pos(cand.pos, Pos(-1, 0)), Dir::W),
        ];

        for (p, d) in next_pos_and_dirs {
            if walls.contains(&p) {
                continue;
            }

            let mut new_path = cand.path.clone();
            new_path.insert(p);
            if cand.dir == d {
                queue.push_back(MoveWithPath {
                    pos: p,
                    dir: d,
                    score: cand.score + 1,
                    path: new_path,
                })
            } else {
                queue.push_back(MoveWithPath {
                    pos: p,
                    dir: d,
                    score: cand.score + 1001,
                    path: new_path,
                })
            }
        }
    }
    Some(tiles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
