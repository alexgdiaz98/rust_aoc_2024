use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;
use std::path::Path;

use anyhow::Result;

use crate::coord::{Coord, ORTHOGONAL_DIRECTIONS};

const WIDTH: isize = 101;
const HEIGHT: isize = 103;
const X_MID: isize = WIDTH / 2;
const X_MID_2: isize = X_MID + 1;
const Y_MID: isize = HEIGHT / 2;
const Y_MID_2: isize = Y_MID + 1;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Robot {
    p: Coord,
    v: Coord,
}

fn read_line(re: &regex::Regex, line: &str) -> (isize, isize, isize, isize) {
    let line_0 = re.captures(line).unwrap();
    let px = line_0.get(1).unwrap().as_str().parse().unwrap();
    let py = line_0.get(2).unwrap().as_str().parse().unwrap();
    let vx = line_0.get(3).unwrap().as_str().parse().unwrap();
    let vy = line_0.get(4).unwrap().as_str().parse().unwrap();
    (px, py, vx, vy)
}

fn sim_second(robots: &mut Vec<Robot>) {
    for robot in robots {
        robot.p.0 = (robot.p.0 + robot.v.0) % WIDTH;
        robot.p.1 = (robot.p.1 + robot.v.1) % HEIGHT;
        if robot.p.0 < 0 {
            robot.p.0 += WIDTH;
        }
        if robot.p.1 < 0 {
            robot.p.1 += HEIGHT;
        }
    }
}

fn count_adjacent(robots: &[Robot]) -> usize {
    let mut count = 0;
    let positions: HashSet<Coord> = robots.iter().map(|r| r.p).collect();
    'p: for position in positions.clone() {
        for direction in ORTHOGONAL_DIRECTIONS {
            if positions.contains(&(position + direction)) {
                count += 1;
                continue 'p;
            }
        }
    }
    count
}

pub fn day14(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let regex = regex::Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    let mut robots: Vec<Robot> = vec![];
    for line in contents.split("\n") {
        let (px, py, vx, vy) = read_line(&regex, line);
        robots.push(Robot {
            p: Coord(px, py),
            v: Coord(vx, vy),
        });
    }
    for _ in 0..100 {
        sim_second(&mut robots);
    }
    for robot in robots.clone() {
        match (robot.p.0, robot.p.1) {
            (..0, _) | (_, ..0) | (WIDTH.., _) | (_, HEIGHT..) => {
                panic!("Overflow Position!");
            }
            (X_MID, _) | (_, Y_MID) => {}
            (..X_MID, ..Y_MID) => q1 += 1,
            (..X_MID, Y_MID_2..) => q2 += 1,
            (X_MID_2.., ..Y_MID) => q3 += 1,
            (X_MID_2.., Y_MID_2..) => q4 += 1,
        }
    }
    let mut t = 100;
    while count_adjacent(&robots.clone()) < 200 {
        t += 1;
        sim_second(&mut robots);
    }
    let positions: HashSet<Coord> = robots.iter().map(|r| r.p).collect();
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            match positions.contains(&Coord(i, j)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
    println!();
    let p1: usize = q1 * q2 * q3 * q4;
    let p2 = t;
    Ok((p1.to_string(), p2.to_string()))
}
