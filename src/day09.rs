use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ChunkType {
    Full(usize),
    Empty,
}

#[derive(Clone, Copy)]
pub struct Chunk {
    len: u8,
    chunk_type: ChunkType,
}

fn checksum(disk_map: &Vec<Chunk>) -> usize {
    let mut i: usize = 0;
    let mut sum = 0;
    for chunk in disk_map {
        let id = match chunk.chunk_type {
            ChunkType::Full(id) => id,
            ChunkType::Empty => {
                i += chunk.len as usize;
                continue;
            }
        };
        for j in 0..chunk.len {
            sum += (i + j as usize) * id;
        }
        i += chunk.len as usize;
    }
    sum
}

fn find_idx_of_id(disk_map: &[Chunk], id: usize) -> Option<usize> {
    for (idx, chunk) in disk_map.iter().enumerate() {
        if chunk.chunk_type == ChunkType::Full(id) {
            return Some(idx);
        }
    }
    None
}

fn find_earliest_gap(disk_map: &[Chunk], len: u8) -> Option<usize> {
    for (idx, chunk) in disk_map.iter().enumerate() {
        if chunk.len >= len && chunk.chunk_type == ChunkType::Empty {
            return Some(idx);
        }
    }
    None
}

fn swap_with_gap(disk_map: &mut Vec<Chunk>, chunk_idx: usize, gap_idx: usize) {
    let chunk = *disk_map.get(chunk_idx).unwrap();
    let space = *disk_map.get(gap_idx).unwrap();
    *disk_map.get_mut(gap_idx).unwrap() = chunk;
    *disk_map.get_mut(chunk_idx).unwrap() = Chunk {
        len: chunk.len,
        chunk_type: ChunkType::Empty,
    };
    // Insert padding if chunk was smaller than gap
    if space.len > chunk.len {
        disk_map.insert(
            gap_idx + 1,
            Chunk {
                len: space.len - chunk.len,
                chunk_type: ChunkType::Empty,
            },
        );
    }
}

fn compact_v1(disk_map: &mut Vec<Chunk>) {
    let mut front_idx = 0;
    let mut back_idx = disk_map.len() - 1;
    while front_idx < back_idx {
        // println!("{:?}", disk_map);
        let mut front_chunk = *disk_map.get(front_idx).unwrap();
        if front_chunk.chunk_type != ChunkType::Empty {
            // Chunk is full. Move on.
            front_idx += 1;
            continue;
        }
        let mut back_chunk = *disk_map.get(back_idx).unwrap();
        if back_chunk.chunk_type == ChunkType::Empty {
            disk_map.remove(back_idx);
            back_idx -= 1;
            continue;
        }
        match front_chunk.len as i8 - back_chunk.len as i8 {
            ..0 => {
                front_chunk.chunk_type = back_chunk.chunk_type;
                back_chunk.len -= front_chunk.len;
                *disk_map.get_mut(front_idx).unwrap() = front_chunk;
                *disk_map.get_mut(back_idx).unwrap() = back_chunk;
                front_idx += 1;
            }
            0 => {
                front_chunk.chunk_type = back_chunk.chunk_type;
                *disk_map.get_mut(front_idx).unwrap() = front_chunk;
                disk_map.remove(back_idx);
                front_idx += 1;
                back_idx -= 1;
            }
            1.. => {
                // More blocks in front chunk
                // Copy all of back block
                // Leave extra space
                *disk_map.get_mut(front_idx).unwrap() = back_chunk;
                disk_map.remove(back_idx);
                disk_map.insert(
                    front_idx + 1,
                    Chunk {
                        len: front_chunk.len - back_chunk.len,
                        chunk_type: ChunkType::Empty,
                    },
                );
            }
        }
    }
}

fn compact_v2(disk_map: &mut Vec<Chunk>, max_id: usize) {
    for id in (0..max_id + 1).rev() {
        let chunk_idx = find_idx_of_id(disk_map.as_slice(), id).unwrap();
        // Chunk to move
        let chunk = *disk_map.get(chunk_idx).unwrap();
        // Index of gap. Skip if no gap exists.
        let gap_idx = match find_earliest_gap(&disk_map.as_slice()[0..chunk_idx], chunk.len) {
            None => continue,
            Some(idx) => idx,
        };
        swap_with_gap(disk_map, chunk_idx, gap_idx);
    }
}

pub fn day09(input_path: &Path) -> Result<(String, String)> {
    let contents: String = read_to_string(input_path)
        .expect("Error reading file")
        .trim()
        .to_string();
    let mut disk_map_1: Vec<Chunk> = vec![];
    let mut disk_map_2: Vec<Chunk> = vec![];
    let mut id = 0;
    for (i, c) in contents.chars().enumerate() {
        let chunk = Chunk {
            len: c.to_digit(10).unwrap() as u8,
            chunk_type: match i % 2 == 0 {
                true => {
                    let new_id = id;
                    id += 1;
                    ChunkType::Full(new_id)
                }
                false => ChunkType::Empty,
            },
        };
        disk_map_1.push(chunk);
        disk_map_2.push(chunk);
    }
    compact_v1(&mut disk_map_1);
    compact_v2(&mut disk_map_2, id - 1);
    let p1 = checksum(&disk_map_1);
    let p2 = checksum(&disk_map_2);

    Ok((p1.to_string(), p2.to_string()))
}
