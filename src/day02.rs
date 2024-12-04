use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

fn is_safe(mut report: Vec<isize>) -> bool {
    if !report.is_sorted() {
        report.reverse();
    }
    if !report.is_sorted() {
        return false;
    }
    let diffs = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| a.abs_diff(*b));
    diffs.clone().min().unwrap() >= 1 && diffs.max().unwrap() <= 3
}

#[allow(dead_code)]
pub fn day02(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    'outer: for line in contents.split("\n") {
        let report: Vec<isize> = line
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();
        if is_safe(report.clone()) {
            p1 += 1;
            p2 += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut r = report.clone();
            r.splice(i..i + 1, []);
            if is_safe(r) {
                p2 += 1;
                continue 'outer;
            }
        }
    }
    Ok((p1.to_string(), p2.to_string()))
}
