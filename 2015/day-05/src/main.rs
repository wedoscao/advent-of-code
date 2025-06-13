use std::fs;

use anyhow::Result;

pub fn contains_retricted(input: &str) -> bool {
    let mut input = input.chars().clone().collect::<Vec<_>>();
    input.reverse();
    while !input.is_empty() {
        let c1 = input.pop().unwrap();
        if let Some(c2) = input.last() {
            match format!("{c1}{c2}").as_str() {
                "ab" | "cd" | "pq" | "xy" => return true,
                _ => {}
            }
        }
    }
    return false;
}

pub fn contains_twice(input: &str) -> bool {
    let mut input = input.chars().clone().collect::<Vec<_>>();
    input.reverse();
    while !input.is_empty() {
        let c1 = input.pop().unwrap();
        if let Some(c2) = input.last() {
            if c1 == *c2 {
                return true;
            }
        }
    }
    return false;
}

pub fn contains_three_vowels(input: &str) -> bool {
    let mut streak = 0;
    for c in input.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => streak += 1,
            _ => {}
        }
    }
    return streak >= 3;
}

fn puzzle_1(input: &str) -> i32 {
    let mut result = 0;

    let strs = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    for s in strs.iter() {
        if !contains_retricted(s) && contains_three_vowels(s) && contains_twice(s) {
            result += 1;
        }
    }
    result
}

pub fn contains_pair(input: &str) -> bool {
    let input = input.chars().collect::<Vec<_>>();
    for (i, c1) in input.iter().enumerate() {
        if let Some(c2) = input.get(i + 1) {
            let slice = input.clone().split_off(i + 2);
            for (j, c3) in slice.iter().enumerate() {
                if *c1 == *c3 {
                    if let Some(c4) = slice.get(j + 1) {
                        if *c2 == *c4 {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

pub fn contains_repeat(input: &str) -> bool {
    let input = input.chars().collect::<Vec<_>>();
    for (i, c1) in input.iter().enumerate() {
        if let Some(c2) = input.get(i + 2) {
            if *c1 == *c2 {
                return true;
            }
        }
    }
    return false;
}

fn puzzle_2(input: &str) -> i32 {
    let mut result = 0;
    let strs = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    for s in strs.iter() {
        let s = *s;
        if contains_pair(s) && contains_repeat(s) {
            result += 1;
        }
    }

    result
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/05.txt")?;
    let input = input.trim();

    let result_1 = puzzle_1(input);
    let result_2 = puzzle_2(input);

    println!("Result 1: {}", result_1);
    println!("Result 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "haegwjzuvuyypxyu";
        assert!(contains_retricted(input))
    }

    #[test]
    fn test_2() {
        let input = "aaa";
        assert!(!contains_retricted(input))
    }

    #[test]
    fn test_3() {
        let input = "ugknbfddgicrmopn";
        assert!(contains_twice(input))
    }

    #[test]
    fn test_4() {
        let input = "jchzalrnumimnmhp";
        assert!(!contains_twice(input))
    }

    #[test]
    fn test_5() {
        let input = "ugknbfddgicrmopn";
        assert!(contains_three_vowels(input))
    }

    #[test]
    fn test_6() {
        let input = "dvszwmarrgswjxmb";
        assert!(!contains_three_vowels(input))
    }

    #[test]
    fn test_7() {
        let input = "uurcxstgmygtbstg";
        assert!(contains_pair(input))
    }

    #[test]
    fn test_8() {
        let input = "ieodomkazucvgmuy";
        assert!(!contains_pair(input))
    }

    #[test]
    fn test_9() {
        let input = "ieodomkazucvgmuy";
        assert!(contains_repeat(input))
    }

    #[test]
    fn test_10() {
        let input = "uurcxstgmygtbstg";
        assert!(!contains_repeat(input))
    }
}
