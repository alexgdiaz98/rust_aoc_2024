use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;
use std::path::Path;

#[allow(dead_code)]
pub fn day01(input_path: &Path) -> (String, String) {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    let mut appearances: HashMap<usize, usize> = HashMap::new();
    for line in contents.split("\n") {
        let mut tokens = line.split_whitespace();
        left.push(tokens.next().unwrap().parse().unwrap());
        let r: usize = tokens.next().unwrap().parse().unwrap();
        right.push(r);
        appearances
            .entry(r)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    left.sort();
    right.sort();
    for (l, r) in zip(left, right) {
        p1 += l.abs_diff(r);
        p2 += l * appearances.get(&l).unwrap_or(&0);
    }
    (p1.to_string(), p2.to_string())
}
