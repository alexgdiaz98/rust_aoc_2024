use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::coord::Coord;

const DIRECTIONS: [Coord; 8] = [
    Coord(0, 1),
    Coord(0, -1),
    Coord(1, 0),
    Coord(-1, 0),
    Coord(-1, -1),
    Coord(-1, 1),
    Coord(1, -1),
    Coord(1, 1),
];

fn find_matches(grid: &HashMap<Coord, char>, coord: Coord, p1: &mut usize, p2: &mut usize) {
    for direction in DIRECTIONS {
        if let (Some('X'), Some('M'), Some('A'), Some('S')) = (
            grid.get(&coord),
            grid.get(&(coord + direction * 1)),
            grid.get(&(coord + direction * 2)),
            grid.get(&(coord + direction * 3)),
        ) {
            *p1 += 1;
        }
    }
    match (
        grid.get(&(coord + Coord(-1, -1))),
        grid.get(&coord),
        grid.get(&(coord + Coord(1, 1))),
    ) {
        (Some('M'), Some('A'), Some('S')) | (Some('S'), Some('A'), Some('M')) => {}
        _ => return,
    }
    match (
        grid.get(&(coord + Coord(-1, 1))),
        grid.get(&coord),
        grid.get(&(coord + Coord(1, -1))),
    ) {
        (Some('M'), Some('A'), Some('S')) | (Some('S'), Some('A'), Some('M')) => {}
        _ => return,
    }
    *p2 += 1;
}

pub fn day04(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut grid: HashMap<Coord, char> = HashMap::new();
    let (mut max_i, mut max_j) = (0, 0);
    for (i, line) in contents.split("\n").enumerate() {
        max_i = max_i.max(i as isize);
        for (j, c) in line.char_indices() {
            max_j = max_j.max(j as isize);
            grid.insert(Coord(i as isize, j as isize), c);
        }
    }
    for i in 0..=max_i {
        for j in 0..=max_j {
            find_matches(&grid, Coord(i, j), &mut p1, &mut p2);
        }
    }
    Ok((p1.to_string(), p2.to_string()))
}
