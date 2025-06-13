use std::{collections::HashMap, fs};

use anyhow::{Result, bail};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    TurnOn((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

impl Instruction {
    fn from_str(input: &str) -> Result<Self> {
        let mut tokens = input.split(' ').clone().collect::<Vec<_>>();
        tokens.reverse();
        if let Some(s1) = tokens.pop() {
            match s1 {
                "toggle" => match (tokens.pop(), tokens.pop(), tokens.pop()) {
                    (Some(s2), Some(s3), Some(s4)) => {
                        if s3 != "through" {
                            bail!("Invalid instruction")
                        }

                        let start = s2
                            .split(',')
                            .map(|s| Ok(s.parse::<usize>()?))
                            .collect::<Result<Vec<_>>>()?;
                        let r1 = match (start.get(0), start.get(1)) {
                            (Some(x), Some(y)) => (*x, *y),
                            _ => bail!("Invalid instruction"),
                        };

                        let end = s4
                            .split(',')
                            .map(|s| Ok(s.parse::<usize>()?))
                            .collect::<Result<Vec<_>>>()?;
                        let r2 = match (end.get(0), end.get(1)) {
                            (Some(x), Some(y)) => (*x, *y),
                            _ => bail!("Invalid instruction"),
                        };

                        return Ok(Self::Toggle(r1, r2));
                    }
                    _ => bail!("Invalid instruction"),
                },
                "turn" => match (tokens.pop(), tokens.pop(), tokens.pop(), tokens.pop()) {
                    (Some(s2), Some(s3), Some(s4), Some(s5)) => {
                        if s4 != "through" {
                            bail!("Invalid instruction")
                        };

                        let start = s3
                            .split(',')
                            .map(|s| Ok(s.parse::<usize>()?))
                            .collect::<Result<Vec<_>>>()?;
                        let r1 = match (start.get(0), start.get(1)) {
                            (Some(x), Some(y)) => (*x, *y),
                            _ => bail!("Invalid instruction"),
                        };

                        let end = s5
                            .split(',')
                            .map(|s| Ok(s.parse::<usize>()?))
                            .collect::<Result<Vec<_>>>()?;
                        let r2 = match (end.get(0), end.get(1)) {
                            (Some(x), Some(y)) => (*x, *y),
                            _ => bail!("Invalid instruction"),
                        };

                        return Ok(match s2 {
                            "on" => Self::TurnOn(r1, r2),
                            "off" => Self::TurnOff(r1, r2),
                            _ => bail!("Invalid instruction"),
                        });
                    }
                    _ => bail!("Invalid instruction"),
                },
                _ => bail!("Invalid instruction"),
            }
        };
        bail!("Invalid instruction")
    }
}

#[derive(Debug)]
pub struct LightGrid {
    grid: HashMap<(usize, usize), bool>,
}

impl LightGrid {
    pub fn new(size: usize) -> Self {
        let mut grid = HashMap::with_capacity((size - 1) * (size - 1));
        for i in 0..size {
            for j in 0..size {
                grid.insert((i, j), false);
            }
        }
        Self { grid }
    }

    pub fn follow(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn((x, y), (a, b)) => {
                for i in *x..=*a {
                    for j in *y..=*b {
                        self.grid.insert((i, j), true);
                    }
                }
            }

            Instruction::TurnOff((x, y), (a, b)) => {
                for i in *x..=*a {
                    for j in *y..=*b {
                        self.grid.insert((i, j), false);
                    }
                }
            }

            Instruction::Toggle((x, y), (a, b)) => {
                for i in *x..=*a {
                    for j in *y..=*b {
                        let prev_val = *self.grid.get(&(i, j)).unwrap();
                        self.grid.insert((i, j), !prev_val);
                    }
                }
            }
        }
    }

    pub fn count_litten(&self) -> usize {
        self.grid.iter().filter(|(_, v)| **v == true).count()
    }
}

pub struct BrightnessGrid {
    grid: HashMap<(usize, usize), usize>,
}

impl BrightnessGrid {
    pub fn new(size: usize) -> Self {
        let mut grid = HashMap::with_capacity((size - 1) * (size - 1));
        for i in 0..size {
            for j in 0..size {
                grid.insert((i, j), 0);
            }
        }
        Self { grid }
    }

    pub fn follow(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn((x, y), (a, b)) => {
                for i in *x..=*a {
                    for j in *y..=*b {
                        let prev_val = *self.grid.get(&(i, j)).unwrap();
                        self.grid.insert((i, j), prev_val + 1);
                    }
                }
            }

            Instruction::TurnOff((x, y), (a, b)) => {
                for i in *x..=*a {
                    for j in *y..=*b {
                        let prev_val = *self.grid.get(&(i, j)).unwrap();
                        if prev_val == 0 {
                            continue;
                        }
                        self.grid.insert((i, j), prev_val - 1);
                    }
                }
            }

            Instruction::Toggle((x, y), (a, b)) => {
                for i in *x..=*a {
                    for j in *y..=*b {
                        let prev_val = *self.grid.get(&(i, j)).unwrap();
                        self.grid.insert((i, j), prev_val + 2);
                    }
                }
            }
        }
    }

    pub fn count_brightness(&self) -> usize {
        self.grid.iter().fold(0, |acc, (_, e)| acc + *e)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/06.txt")?;
    let input = input.trim();

    let mut light_grid = LightGrid::new(1000);
    let mut brightness_grid = BrightnessGrid::new(1000);

    let instructions = input
        .split('\n')
        .map(|s| Ok(Instruction::from_str(s)?))
        .collect::<Result<Vec<_>>>()?;

    for instruction in instructions.iter() {
        light_grid.follow(instruction);
        brightness_grid.follow(instruction);
    }

    let result_1 = light_grid.count_litten();
    let result_2 = brightness_grid.count_brightness();

    println!("Puzzle 1: {}", result_1);
    println!("Puzzle 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "turn on 0,0 through 999,999";
        let expected = Instruction::TurnOn((0, 0), (999, 999));
        assert_eq!(Instruction::from_str(input).unwrap(), expected)
    }

    #[test]

    fn test_2() {
        let input = "toggle 0,0 through 999,0";
        let expected = Instruction::Toggle((0, 0), (999, 0));
        assert_eq!(Instruction::from_str(input).unwrap(), expected)
    }

    #[test]
    fn test_3() {
        let input = "turn off 499,499 through 500,500";
        let expected = Instruction::TurnOff((499, 499), (500, 500));
        assert_eq!(Instruction::from_str(input).unwrap(), expected)
    }
}
