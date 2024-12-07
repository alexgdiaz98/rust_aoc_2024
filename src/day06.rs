use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::coord::Coord;
fn turn(coord: Coord) -> Coord {
    match coord {
        Coord(-1, 0) => Coord(0, 1),
        Coord(0, 1) => Coord(1, 0),
        Coord(1, 0) => Coord(0, -1),
        Coord(0, -1) => Coord(-1, 0),
        _ => coord,
    }
}

fn simulate(
    obstacles: &HashSet<Coord>,
    mut position: Coord,
    max_i: isize,
    max_j: isize,
) -> Option<usize> {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut states: HashSet<(Coord, Coord)> = HashSet::new();
    let mut direction = Coord(-1, 0);
    while position.0 >= 0 && position.1 >= 0 && position.0 <= max_i && position.1 <= max_j {
        if states.contains(&(position, direction)) {
            return None;
        }
        visited.insert(position);
        states.insert((position, direction));
        let dst = position + direction;
        match obstacles.contains(&dst) {
            true => direction = turn(direction),
            false => position = dst,
        }
    }
    Some(visited.len())
}

pub fn day06(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut obstacles: HashSet<Coord> = HashSet::new();
    let mut position: Coord = Coord(0, 0);
    let (mut max_i, mut max_j) = (0, 0);
    for (i, line) in contents.split("\n").enumerate() {
        max_i = max_i.max(i as isize);
        for (j, c) in line.char_indices() {
            max_j = max_j.max(j as isize);
            match c {
                '#' => _ = obstacles.insert(Coord(i as isize, j as isize)),
                '^' => position = Coord(i as isize, j as isize),
                _ => {}
            }
        }
    }
    let p1 = simulate(&obstacles, position, max_i, max_j).unwrap();
    let mut p2: usize = 0;
    for i in 0..=max_i {
        for j in 0..=max_j {
            if Coord(i, j) == position {
                continue;
            }
            let mut new_obstacles = obstacles.clone();
            new_obstacles.insert(Coord(i, j));
            if simulate(&new_obstacles, position, max_i, max_j).is_none() {
                p2 += 1;
            }
        }
    }
    Ok((p1.to_string(), p2.to_string()))
}
