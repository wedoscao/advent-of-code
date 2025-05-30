use std::fs;

use anyhow::Result;

fn puzzle_1(input: &str) -> Result<i64> {
    let mut number = 1;
    loop {
        let input = format!("{}{}", input, number);

        // I am too lazy to reinvent the wheel.
        let hash = md5::compute(input.as_bytes());

        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("00000") {
            break;
        }
        number += 1;
    }
    Ok(number)
}

fn puzzle_2(input: &str) -> Result<i64> {
    let mut number = 1;
    loop {
        let input = format!("{}{}", input, number);

        // I am too lazy to reinvent the wheel.
        let hash = md5::compute(input.as_bytes());

        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("000000") {
            break;
        }
        number += 1;
    }
    Ok(number)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/04.txt")?;
    let input = input.trim();

    let result_1 = puzzle_1(input)?;
    let result_2 = puzzle_2(input)?;

    println!("Puzzle 1: {}", result_1);
    println!("Puzzle 2: {}", result_2);

    Ok(())
}
