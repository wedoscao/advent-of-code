use std::{collections::HashMap, fs};

use anyhow::{Result, bail};

fn puzzle_1(input: &str) -> Result<i64> {
    let mut map = HashMap::<(i64, i64), i64>::new();
    let mut current = (0, 0);
    map.insert(current, 1);

    for direction in input.chars() {
        match direction {
            '>' => current.0 += 1,
            '<' => current.0 -= 1,
            '^' => current.1 += 1,
            'v' => current.1 -= 1,
            _ => bail!("Invalid direction"),
        };
        if let Some(old_times) = map.get(&current) {
            map.insert(current, *old_times + 1);
        } else {
            map.insert(current, 1);
        };
    }

    let result = map.iter().count() as i64;

    Ok(result)
}

fn puzzle_2(input: &str) -> Result<i64> {
    let mut map = HashMap::<(i64, i64), i64>::new();
    let mut current = (0, 0);
    let mut r_current = (0, 0);
    map.insert(current, 1);

    for (i, direction) in input.chars().enumerate() {
        if i % 2 == 0 {
            match direction {
                '>' => current.0 += 1,
                '<' => current.0 -= 1,
                '^' => current.1 += 1,
                'v' => current.1 -= 1,
                _ => bail!("Invalid direction"),
            };
            if let Some(old_times) = map.get(&current) {
                map.insert(current, *old_times + 1);
            } else {
                map.insert(current, 1);
            };
        } else {
            match direction {
                '>' => r_current.0 += 1,
                '<' => r_current.0 -= 1,
                '^' => r_current.1 += 1,
                'v' => r_current.1 -= 1,
                _ => bail!("Invalid direction"),
            };
            if let Some(old_times) = map.get(&current) {
                map.insert(r_current, *old_times + 1);
            } else {
                map.insert(r_current, 1);
            };
        }
    }

    let result = map.iter().count() as i64;

    Ok(result)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/03.txt")?;
    let input = input.trim();

    let result_1 = puzzle_1(input)?;
    let result_2 = puzzle_2(input)?;

    println!("Puzzle 1: {}", result_1);
    println!("Puzzle 2: {}", result_2);
    Ok(())
}
