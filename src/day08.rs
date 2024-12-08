use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

use crate::coord::Coord;

#[inline]
fn is_in_grid(coord: Coord, max_i: isize, max_j: isize) -> bool {
    coord.0 >= 0 && coord.1 >= 0 && coord.0 <= max_i && coord.1 <= max_j
}

pub fn day08(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut freqs: HashMap<char, HashSet<Coord>> = HashMap::new();
    let mut antinodes_p1: HashSet<Coord> = HashSet::new();
    let mut antinodes_p2: HashSet<Coord> = HashSet::new();
    let (mut max_i, mut max_j) = (0, 0);
    for (i, line) in contents.split("\n").enumerate() {
        max_i = max_i.max(i as isize);
        for (j, c) in line.char_indices() {
            max_j = max_j.max(j as isize);
            if c != '.' {
                let coord = Coord(i as isize, j as isize);
                freqs
                    .entry(c)
                    .and_modify(|e| _ = e.insert(coord))
                    .or_insert(HashSet::from([coord]));
            }
        }
    }
    for nodes in freqs.values() {
        for v in nodes.iter().combinations(2) {
            let a = *v.first().unwrap();
            let b = *v.get(1).unwrap();
            let vector_addend = Coord(b.0 - a.0, b.1 - a.1);
            let mut traverser = *b;
            while is_in_grid(traverser, max_i, max_j) {
                if traverser == *b + vector_addend {
                    antinodes_p1.insert(traverser);
                }
                antinodes_p2.insert(traverser);
                traverser += vector_addend;
            }
            traverser = *a;
            while is_in_grid(traverser, max_i, max_j) {
                if traverser == *a - vector_addend {
                    antinodes_p1.insert(traverser);
                }
                antinodes_p2.insert(traverser);
                traverser -= vector_addend;
            }
        }
    }
    let p1 = antinodes_p1.len();
    let p2 = antinodes_p2.len();
    Ok((p1.to_string(), p2.to_string()))
}
