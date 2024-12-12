use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;
use memoize::memoize;

#[memoize]
fn simulate(stone: usize, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }
    if stone == 0 {
        return simulate(1, steps - 1);
    }
    let s = stone.to_string();
    if s.len() % 2 == 0 {
        return simulate(s[..s.len() / 2].parse().unwrap(), steps - 1)
            + simulate(s[s.len() / 2..].parse().unwrap(), steps - 1);
    }
    simulate(stone * 2024, steps - 1)
}

pub fn day11(input_path: &Path) -> Result<(String, String)> {
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let contents: String = read_to_string(input_path).expect("Error reading file");

    for stone in contents.split(" ") {
        p1 += simulate(stone.parse().unwrap(), 25);
        p2 += simulate(stone.parse().unwrap(), 75);
    }
    Ok((p1.to_string(), p2.to_string()))
}
