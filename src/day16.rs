use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

use crate::coord::{Coord, OrthogonalDirection};

type PrimNode = (Coord, OrthogonalDirection);

fn print_maze(path: &HashSet<Coord>, dist: &HashMap<PrimNode, usize>) {
    let max_i = path.iter().max_by_key(|c| c.0).unwrap().0;
    let max_j = path.iter().max_by_key(|c| c.1).unwrap().1;
    for i in 0..=max_i {
        for j in 0..=max_j {
            let coord = Coord(i, j);
            if !path.contains(&coord) {
                print!("#####");
                continue;
            }
            let dist = [
                OrthogonalDirection::UP,
                OrthogonalDirection::RIGHT,
                OrthogonalDirection::DOWN,
                OrthogonalDirection::LEFT,
            ]
            .iter()
            .map(|&dir| dist.get(&(coord, dir)))
            .sorted_by(|a, b| match (a, b) {
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
                (Some(a), Some(b)) => a.cmp(b),
            })
            .next()
            .unwrap();
            match dist {
                None => print!("____ "),
                Some(d) => print!("{: >5}", d),
            }
        }
        println!();
    }
}

fn dijkstras(path: &HashSet<Coord>, src: Coord, dst: Coord) -> usize {
    let mut dist: HashMap<PrimNode, usize> = HashMap::new();
    let mut q: HashSet<PrimNode> = HashSet::new();
    // for coord in path {
    //     q.insert(*coord);
    // }
    q.insert((src, OrthogonalDirection::RIGHT));
    dist.insert((src, OrthogonalDirection::RIGHT), 0);
    while !q.is_empty() {
        // print_maze(path, &dist);
        let u = *q
            .iter()
            .min_by(|a, b| match (dist.get(a), dist.get(b)) {
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(a), Some(b)) => a.cmp(b),
                (None, None) => std::cmp::Ordering::Equal,
            })
            .unwrap();
        q.remove(&u);
        let (u_coord, u_direction) = u;
        let u_dist = *dist.get(&u).unwrap();
        if u.0 == dst {
            return u_dist;
        }
        // print!(
        //     "u = {}, dir = {}, dist = {} -> ",
        //     u_coord, u_direction, u_dist
        // );
        for (v_coord, v_direction, points) in [
            (u_coord + u_direction.coord(), u_direction, 1),
            (u_coord, u_direction.cw(), 1000),
            (u_coord, u_direction.ccw(), 1000),
        ]
        .into_iter()
        .filter(|(coord, _, _)| path.contains(coord))
        {
            let v: PrimNode = (v_coord, v_direction);
            let alt = u_dist + points;
            if path.contains(&v_coord) && alt < *dist.get(&v).unwrap_or(&usize::MAX) {
                // print!("({} is {} facing {}) ", v_coord, alt, v_direction);
                dist.insert(v, alt);
                q.insert(v);
            }
        }
        // println!("\n")
    }
    *[
        OrthogonalDirection::UP,
        OrthogonalDirection::RIGHT,
        OrthogonalDirection::DOWN,
        OrthogonalDirection::LEFT,
    ]
    .iter()
    .map(|&dir| dist.get(&(dst, dir)).unwrap())
    .min()
    .unwrap()
    // *dist
    //     .get(&(dst, OrthogonalDirection::UP))
    //     .or(dist.get(&(dst, OrthogonalDirection::RIGHT)))
    //     .or(dist.get(&(dst, OrthogonalDirection::DOWN)))
    //     .or(dist.get(&(dst, OrthogonalDirection::LEFT)))
    //     .unwrap()
}

pub fn day16(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut p2: usize = 0;
    let mut path: HashSet<Coord> = HashSet::new();
    let mut cursor = Coord(0, 0);
    let mut ending = Coord(0, 0);
    for (i, line) in contents.split("\n").enumerate() {
        for (j, c) in line.char_indices() {
            let coord = Coord(i as isize, j as isize);
            match c {
                'S' => cursor = coord,
                'E' => ending = coord,
                '#' => continue,
                _ => {}
            }
            path.insert(coord);
        }
    }
    let p1 = dijkstras(&path, cursor, ending);
    Ok((p1.to_string(), p2.to_string()))
}
