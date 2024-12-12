use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::coord::Coord;

const DIRECTIONS: [Coord; 4] = [Coord(-1, 0), Coord(0, -1), Coord(1, 0), Coord(0, 1)];

fn dfs(grid: &HashMap<Coord, u8>, start: Coord) -> (usize, usize) {
    println!("Trailhead {:?}", start);
    let mut stack: LinkedList<Coord> = LinkedList::from([start]);
    let mut endings_reached: HashSet<Coord> = HashSet::new();
    let mut distinct_paths: usize = 0;
    while let Some(v) = stack.pop_back() {
        let height = *grid.get(&v).unwrap();
        if height == 9 {
            distinct_paths += 1;
            endings_reached.insert(v);
            println!("Ending {:?} encountered.", v);
        }
        for direction in DIRECTIONS {
            let w = v + direction;
            if let Some(&height_w) = grid.get(&w) {
                if height + 1 == height_w {
                    stack.push_back(v + direction);
                }
            }
        }
    }
    println!("Results: {} {}", endings_reached.len(), distinct_paths);
    (endings_reached.len(), distinct_paths)
}

pub fn day10(input_path: &Path) -> Result<(String, String)> {
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut grid: HashMap<Coord, u8> = HashMap::new();
    let mut trailheads: HashSet<Coord> = HashSet::new();
    for (i, line) in contents.split("\n").enumerate() {
        for (j, c) in line.char_indices() {
            let pos = Coord(i as isize, j as isize);
            grid.insert(pos, c.to_digit(10).unwrap() as u8);
            if c == '0' {
                trailheads.insert(pos);
            }
        }
    }
    for trailhead in trailheads {
        let (endings, paths) = dfs(&grid, trailhead);
        p1 += endings;
        p2 += paths;
    }
    Ok((p1.to_string(), p2.to_string()))
}
