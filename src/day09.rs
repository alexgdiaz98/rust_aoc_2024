use core::fmt;
use std::collections::LinkedList;
use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChunkType {
    Full(usize),
    Empty,
}

#[derive(Clone, Copy)]
pub struct Chunk {
    len: u8,
    chunk_type: ChunkType,
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.chunk_type {
            ChunkType::Empty => write!(f, "{}", self.len),
            ChunkType::Full(id) => write!(f, "\x1b[31m{}x{}\x1b[0m", id, self.len),
        }
    }
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

fn compact_v1(mut disk_map: LinkedList<Chunk>) -> Vec<Chunk> {
    let mut compacted: Vec<Chunk> = vec![];
    while !disk_map.is_empty() {
        let mut cur_block = disk_map.pop_front().unwrap();
        if let ChunkType::Full(_) = cur_block.chunk_type {
            // Normal Block. Populate full blocks.
            compacted.push(cur_block);
            continue;
        };
        // Observe last chunk. Don't pop yet.
        let mut back_block = match disk_map.back_mut() {
            Some(b) => b,
            None => break, // The middle-most block was an empty block. Do nothing.
        };
        // Skip empty chunks in back when encountered.
        if back_block.chunk_type == ChunkType::Empty {
            disk_map.pop_back();
            back_block = match disk_map.back_mut() {
                Some(b) => b,
                None => break, // The middle-most block was an empty block. Do nothing.
            };
        }
        match cur_block.len as i8 - back_block.len as i8 {
            ..0 => {
                // More blocks in back chunk
                compacted.push(Chunk {
                    len: cur_block.len,
                    chunk_type: back_block.chunk_type,
                });
                back_block.len -= cur_block.len;
                continue;
            }
            0 => {
                compacted.push(Chunk {
                    len: back_block.len,
                    chunk_type: back_block.chunk_type,
                });
                disk_map.pop_back();
                continue;
            }
            1.. => {
                // More blocks in front chunk
                compacted.push(Chunk {
                    len: back_block.len,
                    chunk_type: back_block.chunk_type,
                });
                cur_block.len -= back_block.len;
                // Replace empty chunk in front; Pop full block in back;
                disk_map.push_front(cur_block);
                disk_map.pop_back();
                continue;
            }
        }
    }
    compacted
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
    let mut disk_map: LinkedList<Chunk> = LinkedList::new();
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
        disk_map.push_back(chunk);
        disk_map_2.push(chunk);
    }
    let compacted_v1 = compact_v1(disk_map);
    compact_v2(&mut disk_map_2, id - 1);
    let p1 = checksum(&compacted_v1);
    let p2 = checksum(&disk_map_2);

    Ok((p1.to_string(), p2.to_string()))
}
