use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,

    pc: usize,
}

impl Machine {
    pub fn perform_cycle(&mut self, program: &[u8]) -> Option<u64> {
        let op_code = program[self.pc];
        let operand = program[self.pc + 1];

        match op_code {
            0 => {
                self.a /= 2u64.pow(self.get_combo_operand(operand) as u32);
                self.pc += 2;
                None
            }
            1 => {
                self.b ^= operand as u64;
                self.pc += 2;
                None
            }
            2 => {
                self.b = self.get_combo_operand(operand) % 8;
                self.pc += 2;
                None
            }
            3 => {
                if self.a == 0 {
                    self.pc += 2;
                    return None;
                };

                self.pc = operand as usize;
                None
            }
            4 => {
                self.b ^= self.c;
                self.pc += 2;
                None
            }
            5 => {
                self.pc += 2;
                Some(self.get_combo_operand(operand) % 8)
            }
            6 => {
                self.b = self.a / 2u64.pow(self.get_combo_operand(operand) as u32);
                self.pc += 2;
                None
            }
            7 => {
                self.c = self.a / 2u64.pow(self.get_combo_operand(operand) as u32);
                self.pc += 2;
                None
            }
            _ => None,
        }
    }

    fn get_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7.. => panic!("Bad stuff"),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let lines = input.lines().collect_vec();

    let mut machine = Machine {
        a: lines[0].split_once(":").unwrap().1.trim().parse().unwrap(),
        b: lines[1].split_once(":").unwrap().1.trim().parse().unwrap(),
        c: lines[2].split_once(":").unwrap().1.trim().parse().unwrap(),
        pc: 0,
    };

    let program: Vec<u8> = lines[4]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect_vec();

    let mut result: Vec<String> = vec![];
    while machine.pc < program.len() {
        if let Some(out) = machine.perform_cycle(&program) {
            result.push(out.to_string())
        };
    }

    Some(result.join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect_vec();

    let program: Vec<u8> = lines[4]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect_vec();

    dfs(0, &program, 0)
}

fn dfs(a: u64, program: &[u8], depth: u64) -> Option<u64> {
    if depth == program.len() as u64 {
        return Some(a);
    }

    let mut p_copy = program.to_owned();
    p_copy.reverse();

    for i in 0..8 {
        let mut machine = Machine {
            a: a * 8 + i,
            b: 0,
            c: 0,
            pc: 0,
        };
        let mut result: Vec<u8> = vec![];
        while machine.pc < program.len() {
            if let Some(out) = machine.perform_cycle(program) {
                result.push(out as u8)
            };
        }

        if p_copy[depth as usize] == result[0] {
            if let Some(out) = dfs(a * 8 + i, program, depth + 1) {
                return Some(out);
            };
        }
    }

    None
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
