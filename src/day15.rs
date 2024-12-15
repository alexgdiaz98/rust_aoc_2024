use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::coord::{Coord, DOWN, LEFT, RIGHT, UP};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum GridChar {
    Box,
    BoxRight,
    Empty,
    Wall,
}

/// Attempts to push a row of boxes beginning at box_location in the given direction.
/// Returns true if boxes were moved out of the given location. Returns false if the move could not happen.
fn push_boxes(
    grid: &mut HashMap<Coord, GridChar>,
    mut box_location: Coord,
    direction: Coord,
) -> bool {
    let origin = box_location;
    while let Some(&entity) = grid.get(&box_location) {
        match entity {
            GridChar::Empty => break,
            GridChar::Wall => return false,
            GridChar::Box => {}
            _ => panic!("Unexpected value"),
        }
        box_location += direction;
    }
    grid.insert(box_location, GridChar::Box);
    grid.insert(origin, GridChar::Empty);
    true
}

fn push_horizontal(grid: &mut HashMap<Coord, GridChar>, origin: Coord, direction: Coord) -> bool {
    match grid.get(&origin).unwrap() {
        GridChar::Empty => return true,
        GridChar::Wall => return false,
        _ => {}
    };
    let box_to_push = match grid.get(&origin).unwrap() {
        GridChar::BoxRight => origin + LEFT,
        GridChar::Box => origin,
        GridChar::Wall => return false,
        GridChar::Empty => return true,
    };
    let next_location = origin + direction + direction;
    match grid.get(&next_location).unwrap() {
        GridChar::Wall => return false,
        GridChar::Box | GridChar::BoxRight => {
            if !push_horizontal(grid, next_location, direction) {
                return false;
            }
        }
        GridChar::Empty => {}
    };
    grid.insert(box_to_push, GridChar::Empty);
    grid.insert(box_to_push + RIGHT, GridChar::Empty);
    grid.insert(box_to_push + direction, GridChar::Box);
    grid.insert(box_to_push + direction + RIGHT, GridChar::BoxRight);
    true
}

fn can_push_vertical(grid: &HashMap<Coord, GridChar>, origin: Coord, direction: Coord) -> bool {
    let pushing_up = direction == UP;
    let box_to_push = match grid.get(&origin).unwrap() {
        GridChar::BoxRight => origin + Coord(0, -1),
        GridChar::Box => origin,
        GridChar::Wall => return false,
        GridChar::Empty => return true,
    };
    let next_locations: [Coord; 2] = match pushing_up {
        true => [
            box_to_push + Coord(-1, 0),
            box_to_push + Coord(-1, 0) + RIGHT,
        ],
        false => [box_to_push + Coord(1, 0), box_to_push + Coord(1, 0) + RIGHT],
    };
    for next_location in next_locations {
        if !can_push_vertical(grid, next_location, direction) {
            return false;
        }
    }
    true
}

fn push_vertical(grid: &mut HashMap<Coord, GridChar>, origin: Coord, direction: Coord) {
    let pushing_up = direction == UP;
    let box_to_push = match grid.get(&origin).unwrap() {
        GridChar::BoxRight => origin + Coord(0, -1),
        GridChar::Box => origin,
        GridChar::Wall => panic!("Pushing in to a wall"),
        GridChar::Empty => return,
    };
    let next_locations: [Coord; 2] = match pushing_up {
        true => [
            box_to_push + Coord(-1, 0),
            box_to_push + Coord(-1, 0) + RIGHT,
        ],
        false => [box_to_push + Coord(1, 0), box_to_push + Coord(1, 0) + RIGHT],
    };
    for next_location in next_locations {
        push_vertical(grid, next_location, direction);
    }
    grid.insert(box_to_push, GridChar::Empty);
    grid.insert(box_to_push + RIGHT, GridChar::Empty);
    grid.insert(box_to_push + direction, GridChar::Box);
    grid.insert(box_to_push + direction + RIGHT, GridChar::BoxRight);
}

fn calc_gps(grid: &HashMap<Coord, GridChar>) -> usize {
    let mut gps = 0;
    for (&coord, &grid_char) in grid {
        if grid_char == GridChar::Box {
            gps += coord.0 as usize * 100 + coord.1 as usize;
        }
    }
    gps
}

pub fn day15(input_path: &Path) -> Result<(String, String)> {
    let mut grid: HashMap<Coord, GridChar> = HashMap::new();
    let mut grid2: HashMap<Coord, GridChar> = HashMap::new();
    let mut cursor: Coord = Coord(0, 0);
    let mut cursor2: Coord = Coord(0, 0);
    let (mut max_i, mut max_j) = (0, 0);
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let (map, instructions) = contents.split_once("\n\n").unwrap();
    for (i, line) in map.split("\n").enumerate() {
        max_i = max_i.max(i);
        for (j, c) in line.char_indices() {
            max_j = max_j.max(j);
            let coord = Coord(i as isize, j as isize);
            let coord2 = Coord(i as isize, 2 * j as isize);
            let grid_char = match c {
                'O' => GridChar::Box,
                '.' => GridChar::Empty,
                '@' => {
                    cursor = coord;
                    cursor2 = coord2;
                    GridChar::Empty
                }
                '#' => GridChar::Wall,
                _ => panic!("Unexpected character"),
            };
            grid.insert(coord, grid_char);
            grid2.insert(coord2, grid_char);
            if grid_char == GridChar::Box {
                grid2.insert(coord2 + RIGHT, GridChar::BoxRight);
            } else {
                grid2.insert(coord2 + RIGHT, grid_char);
            }
        }
    }
    for instruction in instructions.replace("\n", "").chars() {
        let direction = match instruction {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => panic!("Unexpected instruction"),
        };
        match grid.get(&(cursor + direction)) {
            Some(GridChar::Empty) => cursor += direction,
            _ => {
                if push_boxes(&mut grid, cursor + direction, direction) {
                    cursor += direction;
                }
            }
        }

        match direction {
            LEFT | RIGHT => {
                if push_horizontal(&mut grid2, cursor2 + direction, direction) {
                    cursor2 += direction;
                }
            }
            UP | DOWN => {
                if can_push_vertical(&grid2, cursor2 + direction, direction) {
                    push_vertical(&mut grid2, cursor2 + direction, direction);
                    cursor2 += direction;
                }
            }
            _ => panic!("unexpected direction"),
        };
    }
    let p1 = calc_gps(&grid);
    let p2 = calc_gps(&grid2);
    Ok((p1.to_string(), p2.to_string()))
}
