use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

fn check_ordering(rules: &HashMap<i32, Vec<i32>>, pages: &[i32]) -> Option<usize> {
    let pages = pages.iter();
    for (src, dst) in pages.clone().zip(pages.clone().skip(1)) {
        if let Some(v) = rules.get(dst) {
            if !v.contains(src) {
                return None;
            }
        } else {
            return None;
        }
    }
    let v = pages.collect::<Vec<&i32>>();
    Some(**v.get(v.len() / 2).unwrap() as usize)
}

pub fn day05(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let (rules_inp, pages) = contents.split_once("\n\n").unwrap();
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in rules_inp.split("\n") {
        let (src, dst) = line.split_once('|').unwrap();
        rules
            .entry(dst.parse()?)
            .and_modify(|e| e.push(src.parse().unwrap()))
            .or_insert(vec![src.parse()?]);
    }
    for line in pages.split('\n') {
        let mut pages: Vec<i32> = line.split(',').map(|t| t.parse::<i32>().unwrap()).collect();
        if let Some(addend) = check_ordering(&rules, pages.as_slice()) {
            p1 += addend;
            continue;
        }
        pages.sort_by(|a, b| {
            if let Some(v) = rules.get(b) {
                if v.contains(a) {
                    return Ordering::Less;
                }
            }
            if let Some(v) = rules.get(a) {
                if v.contains(b) {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });
        if let Some(addend) = check_ordering(&rules, pages.as_slice()) {
            p2 += addend;
        }
    }
    Ok((p1.to_string(), p2.to_string()))
}
