use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    thread::current,
};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Robot {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = input.lines().map(|line| {
        let (pos_str, v_str) = line.split_once(" ").unwrap();
        let (x_str, y_str) = pos_str[2..].split_once(",").unwrap();
        let (v_x_str, v_y_str) = v_str[2..].split_once(",").unwrap();

        Robot {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
            v_x: v_x_str.parse().unwrap(),
            v_y: v_y_str.parse().unwrap(),
        }
    });

    let final_robots = robots.into_iter().map(|r| do_steps(&r, 100));

    let mut quad1 = 0;
    let mut quad2 = 0;
    let mut quad3 = 0;
    let mut quad4 = 0;

    for robot in final_robots {
        match get_quadrant(&robot) {
            1 => quad1 += 1,
            2 => quad2 += 1,
            3 => quad3 += 1,
            4 => quad4 += 1,
            _ => {}
        }
    }

    Some(quad1 * quad2 * quad3 * quad4)
}

fn do_steps(robot: &Robot, steps: i64) -> Robot {
    let new_x = (robot.x + robot.v_x * steps).rem_euclid(101);
    let new_y = (robot.y + robot.v_y * steps).rem_euclid(103);

    Robot {
        x: new_x,
        y: new_y,
        v_x: robot.v_x,
        v_y: robot.v_y,
    }
}

fn get_quadrant(robot: &Robot) -> u32 {
    let mid_x = 50;
    let mid_y = 51;

    match (robot.x.cmp(&mid_x), robot.y.cmp(&mid_y)) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 1,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 2,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 3,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 4,
        (_, std::cmp::Ordering::Equal) => 0,
        (std::cmp::Ordering::Equal, _) => 0,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = input
        .lines()
        .map(|line| {
            let (pos_str, v_str) = line.split_once(" ").unwrap();
            let (x_str, y_str) = pos_str[2..].split_once(",").unwrap();
            let (v_x_str, v_y_str) = v_str[2..].split_once(",").unwrap();

            Robot {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
                v_x: v_x_str.parse().unwrap(),
                v_y: v_y_str.parse().unwrap(),
            }
        })
        .collect_vec();

    // Commenting out for tests as don't want CI drawing a ton
    // fs::create_dir_all("./data/results").unwrap();
    let mut current_safe_counts = 0;
    for step in 0..101 * 103 {
        let next_robots = robots.iter().map(do_step).collect_vec();

        let mut quad1 = 0;
        let mut quad2 = 0;
        let mut quad3 = 0;
        let mut quad4 = 0;

        for robot in next_robots.iter() {
            match get_quadrant(robot) {
                1 => quad1 += 1,
                2 => quad2 += 1,
                3 => quad3 += 1,
                4 => quad4 += 1,
                _ => {}
            }
        }

        let safety = quad1 * quad2 * quad3 * quad4;
        if current_safe_counts == 0 {
            current_safe_counts = safety
        }

        if safety < current_safe_counts {
            current_safe_counts = safety;
            // Commenting out for tests as don't want CI drawing a ton
            // draw(&next_robots, step + 1);
        };

        robots = next_robots;
    }
    None
}

fn do_step(robot: &Robot) -> Robot {
    let new_x = (robot.x + robot.v_x).rem_euclid(101);
    let new_y = (robot.y + robot.v_y).rem_euclid(103);

    Robot {
        x: new_x,
        y: new_y,
        v_x: robot.v_x,
        v_y: robot.v_y,
    }
}

fn draw(robots: &[Robot], step: i32) {
    let positions = robots.iter().fold(HashSet::new(), |mut acc, r| {
        acc.insert((r.x, r.y));
        acc
    });

    let mut lines = vec![];
    for y in 0..103 {
        let mut line = String::new();
        for x in 0..101 {
            if positions.contains(&(x, y)) {
                line.push('X');
            } else {
                line.push('.')
            }
        }
        lines.push(line)
    }

    let drawing = lines.join("\n");
    let file_path = PathBuf::from("./data/results").join(step.to_string());
    let mut file = File::create(file_path).unwrap();
    file.write_all(&drawing.into_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
