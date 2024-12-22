use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(18);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(i32, i32);

fn sum_pos(pos1: Pos, pos2: Pos) -> Pos {
    Pos(pos1.0 + pos2.0, pos1.1 + pos2.1)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Move {
    pos: Pos,
    score: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut walls = HashSet::new();
    let end = Pos(70, 70);
    for (count, line) in input.lines().enumerate() {
        if count >= 1024 {
            break;
        };

        let (x, y) = line.split_once(",").unwrap();
        walls.insert(Pos(x.parse().unwrap(), y.parse().unwrap()));
    }
    let mut min = 1000000;

    let mut visited: HashMap<Pos, u32> = HashMap::new();
    let mut queue = VecDeque::from_iter(vec![Move {
        pos: Pos(0, 0),
        score: 0,
    }]);

    while let Some(cand) = queue.pop_front() {
        // handle end case
        if cand.pos == end && cand.score < min {
            min = cand.score;
            continue;
        }

        // handle already seen but better
        if let Some(cached) = visited.get(&(cand.pos)) {
            if cached <= &cand.score {
                continue;
            }
        };
        *visited.entry(cand.pos).or_default() = cand.score;

        let next_pos = vec![
            (sum_pos(cand.pos, Pos(0, -1))),
            (sum_pos(cand.pos, Pos(1, 0))),
            (sum_pos(cand.pos, Pos(0, 1))),
            (sum_pos(cand.pos, Pos(-1, 0))),
        ];

        for p in next_pos {
            if walls.contains(&p) || p.0 > end.0 || p.0 < 0 || p.1 > end.1 || p.1 < 0 {
                continue;
            }

            queue.push_back(Move {
                pos: p,
                score: cand.score + 1,
            })
        }
    }
    Some(min)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut walls = HashSet::new();
    let mut future_walls = VecDeque::new();
    let end = Pos(70, 70);
    for (count, line) in input.lines().enumerate() {
        let (x, y) = line.split_once(",").unwrap();
        if count < 1024 {
            walls.insert(Pos(x.parse().unwrap(), y.parse().unwrap()));
        } else {
            future_walls.push_back(Pos(x.parse().unwrap(), y.parse().unwrap()))
        };
    }

    let mut res = String::new();
    loop {
        let mut min = 1000000;

        let mut visited: HashMap<Pos, u32> = HashMap::new();
        let mut queue = VecDeque::from_iter(vec![Move {
            pos: Pos(0, 0),
            score: 0,
        }]);

        while let Some(cand) = queue.pop_front() {
            // handle end case
            if cand.pos == end && cand.score < min {
                min = cand.score;
                continue;
            }

            // handle already seen but better
            if let Some(cached) = visited.get(&(cand.pos)) {
                if cached <= &cand.score {
                    continue;
                }
            };
            *visited.entry(cand.pos).or_default() = cand.score;

            let next_pos = vec![
                (sum_pos(cand.pos, Pos(0, -1))),
                (sum_pos(cand.pos, Pos(1, 0))),
                (sum_pos(cand.pos, Pos(0, 1))),
                (sum_pos(cand.pos, Pos(-1, 0))),
            ];

            for p in next_pos {
                if walls.contains(&p) || p.0 > end.0 || p.0 < 0 || p.1 > end.1 || p.1 < 0 {
                    continue;
                }

                queue.push_back(Move {
                    pos: p,
                    score: cand.score + 1,
                })
            }
        }

        if min == 1000000 {
            break;
        } else if let Some(next_fallen) = future_walls.pop_front() {
            res = format!("{},{}", next_fallen.0, next_fallen.1);
            walls.insert(next_fallen);
        } else {
            break;
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(146));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("".into()));
    }
}
