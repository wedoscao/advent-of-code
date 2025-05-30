use std::{cmp, fs};

use anyhow::Result;

fn puzzle_1(input: &str) -> Result<i64> {
    let mut result = 0;
    let presents = input.split('\n').collect::<Vec<_>>();
    for present in presents.iter() {
        let present = present.split('x').collect::<Vec<_>>();
        let x = present.get(0).unwrap().parse::<i64>()?;
        let y = present.get(1).unwrap().parse::<i64>()?;
        let z = present.get(2).unwrap().parse::<i64>()?;
        let surface_area = 2 * x * y + 2 * x * z + 2 * y * z;
        let min = { cmp::min(x * y, cmp::min(x * z, y * z)) };
        result += surface_area + min;
    }
    Ok(result)
}

fn puzzle_2(input: &str) -> Result<i64> {
    let mut result = 0;
    let presents = input.split('\n').collect::<Vec<_>>();
    for present in presents.iter() {
        let present = present.split('x').collect::<Vec<_>>();
        let x = present.get(0).unwrap().parse::<i64>()?;
        let y = present.get(1).unwrap().parse::<i64>()?;
        let z = present.get(2).unwrap().parse::<i64>()?;
        let min_perimeter = cmp::min(2 * x + 2 * y, cmp::min(2 * x + 2 * z, 2 * y + 2 * z));
        let bow = x * y * z;
        result += min_perimeter + bow;
    }
    Ok(result)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/02.txt")?;
    let input = input.trim();

    let result_1 = puzzle_1(input)?;
    let result_2 = puzzle_2(input)?;

    println!("Puzzle 1: {}", result_1);
    println!("Puzzle 2: {}", result_2);

    Ok(())
}
