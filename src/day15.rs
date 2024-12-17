use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::coord::{
    Coord, HorizontalDirection, OrthogonalDirection, ToCoord, VerticalDirection, LEFT, RIGHT,
};

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
    direction: OrthogonalDirection,
) -> bool {
    let origin = box_location;
    while let Some(&entity) = grid.get(&box_location) {
        match entity {
            GridChar::Empty => break,
            GridChar::Wall => return false,
            GridChar::Box => {}
            _ => panic!("Unexpected value"),
        }
        box_location += direction.coord();
    }
    grid.insert(box_location, GridChar::Box);
    grid.insert(origin, GridChar::Empty);
    true
}

fn push_horizontal(
    grid: &mut HashMap<Coord, GridChar>,
    origin: Coord,
    direction: HorizontalDirection,
) -> bool {
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
    let next_location = origin + (direction.coord() * 2);
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
    grid.insert(box_to_push + direction.coord(), GridChar::Box);
    grid.insert(box_to_push + direction.coord() + RIGHT, GridChar::BoxRight);
    true
}

fn can_push_vertical(
    grid: &HashMap<Coord, GridChar>,
    origin: Coord,
    direction: VerticalDirection,
) -> bool {
    let box_to_push = match grid.get(&origin).unwrap() {
        GridChar::BoxRight => origin + Coord(0, -1),
        GridChar::Box => origin,
        GridChar::Wall => return false,
        GridChar::Empty => return true,
    };
    let next_locations: [Coord; 2] = [
        box_to_push + direction.coord(),
        box_to_push + direction.coord() + RIGHT,
    ];
    for next_location in next_locations {
        if !can_push_vertical(grid, next_location, direction) {
            return false;
        }
    }
    true
}

fn push_vertical(grid: &mut HashMap<Coord, GridChar>, origin: Coord, direction: VerticalDirection) {
    let box_to_push = match grid.get(&origin).unwrap() {
        GridChar::BoxRight => origin + Coord(0, -1),
        GridChar::Box => origin,
        GridChar::Wall => panic!("Pushing in to a wall"),
        GridChar::Empty => return,
    };
    let next_locations: [Coord; 2] = [
        box_to_push + direction.coord(),
        box_to_push + direction.coord() + RIGHT,
    ];
    for next_location in next_locations {
        push_vertical(grid, next_location, direction);
    }
    grid.insert(box_to_push, GridChar::Empty);
    grid.insert(box_to_push + RIGHT, GridChar::Empty);
    grid.insert(box_to_push + direction.coord(), GridChar::Box);
    grid.insert(box_to_push + direction.coord() + RIGHT, GridChar::BoxRight);
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
            '^' => OrthogonalDirection::UP,
            '>' => OrthogonalDirection::RIGHT,
            'v' => OrthogonalDirection::DOWN,
            '<' => OrthogonalDirection::LEFT,
            _ => panic!("Unexpected instruction"),
        };
        match grid.get(&(cursor + direction.coord())) {
            Some(GridChar::Empty) => cursor += direction.coord(),
            _ => {
                if push_boxes(&mut grid, cursor + direction.coord(), direction) {
                    cursor += direction.coord();
                }
            }
        }

        match direction {
            OrthogonalDirection::LEFT | OrthogonalDirection::RIGHT => {
                let horizontal_direction: HorizontalDirection = direction.into();
                if push_horizontal(
                    &mut grid2,
                    cursor2 + direction.coord(),
                    horizontal_direction,
                ) {
                    cursor2 += direction.coord();
                }
            }
            OrthogonalDirection::UP | OrthogonalDirection::DOWN => {
                let vertical_direction: VerticalDirection = direction.into();
                if can_push_vertical(&grid2, cursor2 + direction.coord(), vertical_direction) {
                    push_vertical(&mut grid2, cursor2 + direction.coord(), vertical_direction);
                    cursor2 += direction.coord();
                }
            }
        };
    }
    let p1 = calc_gps(&grid);
    let p2 = calc_gps(&grid2);
    Ok((p1.to_string(), p2.to_string()))
}
