use itertools::Itertools;

advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];
const SAM: [char; 3] = ['S', 'A', 'M'];

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let y_range = matrix.len();
    let x_range = matrix[0].len();

    let mut found = 0;
    for y in 0..y_range {
        for x in 0..x_range {
            if check_north(&matrix, x, y) {
                found += 1;
            };

            if check_northeast(&matrix, x, y) {
                found += 1;
            };

            if check_east(&matrix, x, y) {
                found += 1;
            };

            if check_southeast(&matrix, x, y) {
                found += 1;
            };

            if check_south(&matrix, x, y) {
                found += 1;
            };

            if check_southwest(&matrix, x, y) {
                found += 1;
            };

            if check_west(&matrix, x, y) {
                found += 1;
            };

            if check_northwest(&matrix, x, y) {
                found += 1;
            };
        }
    }
    Some(found)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let y_range = matrix.len();
    let x_range = matrix[0].len();

    let mut found = 0;
    for y in 0..y_range {
        for x in 0..x_range {
            if check_x_mas(&matrix, x, y) {
                found += 1;
            };
        }
    }
    Some(found)
}

fn check_north(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if y < 3 {
        return false;
    }
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test_col) = matrix.get(y - idx) {
            if &to_test_col[x] != test_char {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_northeast(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if y < 3 {
        return false;
    }
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test_col) = matrix.get(y - idx) {
            if let Some(to_test) = to_test_col.get(x + idx) {
                if to_test != test_char {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_east(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test) = matrix[y].get(x + idx) {
            if to_test != test_char {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_southeast(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test_col) = matrix.get(y + idx) {
            if let Some(to_test) = to_test_col.get(x + idx) {
                if to_test != test_char {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_south(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test_col) = matrix.get(y + idx) {
            if &to_test_col[x] != test_char {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_southwest(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 {
        return false;
    }
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test_col) = matrix.get(y + idx) {
            if let Some(to_test) = to_test_col.get(x - idx) {
                if to_test != test_char {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_west(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 {
        return false;
    }
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test) = matrix[y].get(x - idx) {
            if to_test != test_char {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_northwest(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 3 || y < 3 {
        return false;
    }
    for (idx, test_char) in XMAS.iter().enumerate() {
        if let Some(to_test_col) = matrix.get(y - idx) {
            if let Some(to_test) = to_test_col.get(x - idx) {
                if to_test != test_char {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn check_x_mas(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    if x < 1 || y < 1 || y > matrix.len() - 2 || x > matrix[0].len() - 2 {
        return false;
    }

    let word1 = [matrix[y - 1][x - 1], matrix[y][x], matrix[y + 1][x + 1]];
    let word2 = [matrix[y + 1][x - 1], matrix[y][x], matrix[y - 1][x + 1]];

    (word1 == SAM || word1 == MAS) && (word2 == MAS || word2 == SAM)
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
