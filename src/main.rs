pub mod coord;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
pub mod matrix;

use std::{env, ffi::OsString, path::Path, process::exit};

use anyhow::Result;
use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;
use day07::day07;
use day08::day08;
use day09::day09;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;
use day16::day16;

fn main() -> Result<()> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() != 1 && args.len() != 2 {
        eprintln!(
            "Proper usage: 'cargo run {{day}} {{path_to_input}}' ({} arguments detected)",
            args.len()
        );
        exit(-1);
    }
    let day = args
        .first()
        .unwrap()
        .to_owned()
        .into_string()
        .expect("Error parsing day token.")
        .parse::<u8>()
        .expect("Error converting day token to number.");
    if day == 0 || day > 25 {
        eprintln!("Provided day is not between 1-25.");
        exit(-1);
    }
    let default_input_path = OsString::from(format!("../inputs/{:0>2}.txt", day.to_string()));
    let input_path = Path::new(args.get(1).unwrap_or(&default_input_path));
    println!("Day: {:0>2} Input Path: {:?}", day.to_string(), input_path);
    let (p1, p2) = match day {
        1 => day01(input_path),
        2 => day02(input_path),
        3 => day03(input_path),
        4 => day04(input_path),
        5 => day05(input_path),
        6 => day06(input_path),
        7 => day07(input_path),
        8 => day08(input_path),
        9 => day09(input_path),
        10 => day10(input_path),
        11 => day11(input_path),
        12 => day12(input_path),
        13 => day13(input_path),
        14 => day14(input_path),
        15 => day15(input_path),
        16 => day16(input_path),
        _ => {
            eprintln!("Invalid day: {}", day);
            exit(-1);
        }
    }?;
    println!("p1:\n{}\np2:\n{}", p1, p2);
    Ok(())
}
