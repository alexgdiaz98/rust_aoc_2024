use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;
use disjoint::DisjointSet;

use crate::coord::{Coord, ORTHOGONAL_DIRECTIONS};

#[inline]
fn coord_to_idx(coord: Coord, max_j: usize) -> usize {
    coord.0 as usize * (max_j + 1) + coord.1 as usize
}

#[inline]
fn idx_to_coord(idx: usize, max_j: usize) -> Coord {
    let i = idx / (max_j + 1);
    let j = idx % (max_j + 1);
    Coord(i as isize, j as isize)
}

fn get_perimeter(region: &[Coord]) -> usize {
    let positions: HashSet<Coord> = region.iter().copied().collect();
    let mut perimeter = 0;
    for &p in region {
        for direction in ORTHOGONAL_DIRECTIONS {
            if !positions.contains(&(p + direction)) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn get_sides(region: &[Coord]) -> usize {
    let mut sides = 0;
    let positions: HashSet<Coord> = region.iter().copied().collect();
    let i_min = region.iter().min_by_key(|c| c.0).unwrap().0;
    let i_max = region.iter().max_by_key(|c| c.0).unwrap().0;
    let j_min = region.iter().min_by_key(|c| c.1).unwrap().1;
    let j_max = region.iter().max_by_key(|c| c.1).unwrap().1;
    for i in i_min..=i_max {
        let mut checking_top_side = false;
        let mut checking_bottom_side = false;
        for j in j_min..=j_max {
            let coord = Coord(i, j);
            if positions.contains(&coord) && !positions.contains(&(coord + Coord(-1, 0))) {
                checking_top_side = true;
            } else if checking_top_side {
                checking_top_side = false;
                sides += 1;
            }
            if positions.contains(&coord) && !positions.contains(&(coord + Coord(1, 0))) {
                checking_bottom_side = true;
            } else if checking_bottom_side {
                checking_bottom_side = false;
                sides += 1;
            }
        }
        if checking_top_side {
            // Side continued to end
            sides += 1;
        }
        if checking_bottom_side {
            sides += 1;
        }
    }
    for j in j_min..=j_max {
        let mut checking_left_side = false;
        let mut checking_right_side = false;
        for i in i_min..=i_max {
            let coord = Coord(i, j);
            if positions.contains(&coord) && !positions.contains(&(coord + Coord(0, -1))) {
                checking_left_side = true;
            } else if checking_left_side {
                checking_left_side = false;
                sides += 1;
            }
            if positions.contains(&coord) && !positions.contains(&(coord + Coord(0, 1))) {
                checking_right_side = true;
            } else if checking_right_side {
                checking_right_side = false;
                sides += 1;
            }
        }
        if checking_left_side {
            // Side continued to end
            sides += 1;
        }
        if checking_right_side {
            sides += 1;
        }
    }
    sides
}

pub fn day12(input_path: &Path) -> Result<(String, String)> {
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let max_i = lines.len() - 1;
    let max_j = lines.first().unwrap().len() - 1;
    let mut regions: DisjointSet = DisjointSet::new();
    let mut grid: HashMap<Coord, char> = HashMap::new();
    let mut areas: HashMap<Coord, usize> = HashMap::new();
    for (i, &line) in lines.iter().enumerate() {
        for (j, c) in line.char_indices() {
            let cursor = Coord(i as isize, j as isize);
            regions.add_singleton();
            grid.insert(cursor, c);
            areas.entry(cursor).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    for i in 0..max_i + 1 {
        for j in 0..max_j + 1 {
            let u = Coord(i as isize, j as isize);
            let plant = *grid.get(&u).unwrap();
            let ds_idx = coord_to_idx(u, max_j);

            let right_coord = u + Coord(0, 1);
            let right_plant = grid.get(&right_coord);
            let ds_right_idx = coord_to_idx(right_coord, max_j);
            if Some(&plant) == right_plant {
                regions.join(ds_idx, ds_right_idx);
            }

            let down_coord = u + Coord(1, 0);
            let down_plant = grid.get(&down_coord);
            let ds_down_idx = coord_to_idx(down_coord, max_j);
            if Some(&plant) == down_plant {
                regions.join(ds_idx, ds_down_idx);
            }
        }
    }
    for region in regions.sets() {
        let coords: Vec<Coord> = region.iter().map(|&idx| idx_to_coord(idx, max_j)).collect();
        let area = coords.len();
        let perimeter = get_perimeter(&coords);
        let sides = get_sides(&coords);
        p1 += area * perimeter;
        p2 += area * sides;
    }
    Ok((p1.to_string(), p2.to_string()))
}
