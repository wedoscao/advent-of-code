use std::fs;

use anyhow::{Result, bail};

fn puzzle_1(input: &str) -> Result<i64> {
    let mut result = 0;
    for c in input.chars() {
        match c {
            '(' => result += 1,
            ')' => result -= 1,
            _ => bail!("Invalid input"),
        }
    }
    Ok(result)
}

fn puzzle_2(input: &str) -> Result<i64> {
    let mut result = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => result += 1,
            ')' => result -= 1,
            _ => bail!("Invalid input"),
        };
        if result == -1 {
            return Ok(i as i64 + 1);
        }
    }
    Ok(-1)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/01.txt")?;
    let input = input.trim();

    let result_1 = puzzle_1(input)?;
    let result_2 = puzzle_2(input)?;

    println!("Puzzle 1: {}", result_1);
    println!("Puzzle 2: {}", result_2);
    Ok(())
}
