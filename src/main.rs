mod day01;
mod day02;
mod day03;

use std::{env, ffi::OsString, path::Path, process::exit};

use anyhow::Result;
use day01::day01;
use day02::day02;
use day03::day03;

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
        _ => {
            eprintln!("Invalid day: {}", day);
            exit(-1);
        }
    }?;
    println!("p1:\n{}\np2:\n{}", p1, p2);
    Ok(())
}
