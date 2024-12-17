use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::coord::{Coord, OrthogonalDirection};

type PrimNode = (Coord, OrthogonalDirection);

fn dijkstras(path: &HashSet<Coord>, src: Coord, dst: Coord) -> (usize, usize) {
    let mut dist: HashMap<PrimNode, usize> = HashMap::new();
    let mut q: HashSet<PrimNode> = HashSet::new();
    let mut path_to_v: HashMap<PrimNode, HashSet<Coord>> = HashMap::new();
    q.insert((src, OrthogonalDirection::RIGHT));
    dist.insert((src, OrthogonalDirection::RIGHT), 0);
    path_to_v.insert((src, OrthogonalDirection::RIGHT), HashSet::from([src]));
    while !q.is_empty() {
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
            // End condition. This is the fastest path to the end.
            return (u_dist, path_to_v.get(&u).unwrap().len());
        }
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
                dist.insert(v, alt);
                q.insert(v);
                let mut new_set = path_to_v.get(&u).unwrap().clone();
                new_set.insert(v_coord);
                path_to_v.insert(v, new_set);
            } else if path.contains(&v_coord) && alt == *dist.get(&v).unwrap_or(&usize::MAX) {
                let mut new_set = path_to_v.get(&u).unwrap().clone();
                new_set.insert(v_coord);
                path_to_v
                    .entry(v)
                    .and_modify(|e| e.extend(new_set.iter()))
                    .or_insert(new_set);
            }
        }
    }
    panic!("Did not reach end");
}

pub fn day16(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path).expect("Error reading file");
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
    let (p1, p2) = dijkstras(&path, cursor, ending);
    Ok((p1.to_string(), p2.to_string()))
}
