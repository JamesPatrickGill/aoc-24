use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

pub fn part_one(input: &str) -> Option<i64> {
    let equations = input
        .lines()
        .map(|line| {
            let (target_str, numbers_str) = line.split_once(": ").unwrap();
            let numbers: Vec<i64> = numbers_str
                .split(" ")
                .map(|num_str| num_str.parse().unwrap())
                .collect_vec();
            Equation {
                numbers,
                target: target_str.parse().unwrap(),
            }
        })
        .collect_vec();

    let result: i64 = equations.into_iter().fold(0, |acc, eq| {
        if solve_equation(&eq).is_some() {
            acc + eq.target
        } else {
            acc
        }
    });

    Some(result)
}

fn solve_equation(equation: &Equation) -> Option<HashSet<Vec<char>>> {
    let mut solutions = HashSet::new();
    if equation.numbers.len() == 2 {
        if equation.numbers[0] + equation.numbers[1] == equation.target {
            solutions.insert(vec!['+'].into());
        };

        if equation.numbers[0] * equation.numbers[1] == equation.target {
            solutions.insert(vec!['*'].into());
        };

        return if solutions.is_empty() {
            None
        } else {
            Some(solutions)
        };
    }

    let mut numbers_clone = equation.numbers.clone();
    let last = numbers_clone.pop().unwrap();

    if last < equation.target {
        let add_eq_target = equation.target - last;
        let add_smaller_eq = Equation {
            target: add_eq_target,
            numbers: numbers_clone.clone(),
        };
        let add_smaller_solutions = solve_equation(&add_smaller_eq);
        if let Some(sols) = add_smaller_solutions {
            for sol in sols {
                let mut new_sol = sol.clone();
                new_sol.push('+');
                solutions.insert(new_sol);
            }
        }
    }

    if equation.target % last == 0 {
        let mult_eq_target = equation.target / last;
        let mult_smaller_eq = Equation {
            target: mult_eq_target,
            numbers: numbers_clone.clone(),
        };
        let mult_smaller_solutions = solve_equation(&mult_smaller_eq);
        if let Some(sols) = mult_smaller_solutions {
            for sol in sols {
                let mut new_sol = sol.clone();
                new_sol.push('*');
                solutions.insert(new_sol);
            }
        };
    }

    if solutions.is_empty() {
        None
    } else {
        Some(solutions)
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let equations = input
        .lines()
        .map(|line| {
            let (target_str, numbers_str) = line.split_once(": ").unwrap();
            let numbers: Vec<i64> = numbers_str
                .split(" ")
                .map(|num_str| num_str.parse().unwrap())
                .collect_vec();
            Equation {
                numbers,
                target: target_str.parse().unwrap(),
            }
        })
        .collect_vec();

    let result: i64 = equations.into_iter().fold(0, |acc, eq| {
        if solve_equation_2(&eq).is_some() {
            acc + eq.target
        } else {
            acc
        }
    });

    Some(result)
}

fn solve_equation_2(equation: &Equation) -> Option<HashSet<VecDeque<char>>> {
    let mut solutions = HashSet::new();
    if equation.numbers.len() == 2 {
        if equation.numbers[0] + equation.numbers[1] == equation.target {
            solutions.insert(vec!['+'].into());
        };

        if equation.numbers[0] * equation.numbers[1] == equation.target {
            solutions.insert(vec!['*'].into());
        };

        let concat = format!("{}{}", equation.numbers[0], equation.numbers[1])
            .parse::<i64>()
            .unwrap();
        if concat == equation.target {
            solutions.insert(vec!['|'].into());
        };

        return if solutions.is_empty() {
            None
        } else {
            Some(solutions)
        };
    }

    let mut numbers_clone = equation.numbers.clone();
    let last = numbers_clone.pop().unwrap();

    if last < equation.target {
        let add_eq_target = equation.target - last;
        let add_smaller_eq = Equation {
            target: add_eq_target,
            numbers: numbers_clone.clone(),
        };
        let add_smaller_solutions = solve_equation_2(&add_smaller_eq);
        if let Some(sols) = add_smaller_solutions {
            for sol in sols {
                let mut new_sol = sol.clone();
                new_sol.push_back('+');
                solutions.insert(new_sol);
            }
        }
    };

    if equation.target % last == 0 {
        let mult_eq_target = equation.target / last;
        let mult_smaller_eq = Equation {
            target: mult_eq_target,
            numbers: numbers_clone.clone(),
        };
        let mult_smaller_solutions = solve_equation_2(&mult_smaller_eq);
        if let Some(sols) = mult_smaller_solutions {
            for sol in sols {
                let mut new_sol = sol.clone();
                new_sol.push_back('*');
                solutions.insert(new_sol);
            }
        };
    };

    let target_as_str = equation.target.to_string();
    if target_as_str.ends_with(&last.to_string()) {
        let new_target = &target_as_str[..target_as_str.len() - last.to_string().len()];
        if !new_target.is_empty() {
            let concat_smaller_eq = Equation {
                target: new_target.parse().unwrap(),
                numbers: numbers_clone.clone(),
            };
            let concat_smaller_solutions = solve_equation_2(&concat_smaller_eq);
            if let Some(sols) = concat_smaller_solutions {
                for sol in sols {
                    let mut new_sol = sol.clone();
                    new_sol.push_back('|');
                    solutions.insert(new_sol);
                }
            };
        };
    };
    if solutions.is_empty() {
        None
    } else {
        Some(solutions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
