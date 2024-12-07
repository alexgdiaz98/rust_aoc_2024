use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;
use itertools::{repeat_n, Itertools};

enum Operand {
    Add,
    Mul,
    Concat,
}

fn check_equation(result: i64, values: &[i64], ops: &[&Operand]) -> bool {
    let mut running = *values.first().unwrap();
    for (i, value) in values.iter().skip(1).enumerate() {
        match ops.get(i).unwrap() {
            Operand::Add => running += value,
            Operand::Mul => running *= value,
            Operand::Concat => {
                running *= 10_i64.pow(value.to_string().len() as u32);
                running += value;
            }
        }
    }
    running == result
}

pub fn day07(input_path: &Path) -> Result<(String, String)> {
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let contents: String = read_to_string(input_path).expect("Error reading file");
    for line in contents.split("\n") {
        let (result, values) = line.split_once(':').unwrap();
        let result: i64 = result.parse().unwrap();
        let values: Vec<i64> = values
            .trim()
            .split(' ')
            .map(|v| v.parse().unwrap())
            .collect();
        for ops in repeat_n([Operand::Add, Operand::Mul].iter(), values.len() - 1)
            .multi_cartesian_product()
        {
            if check_equation(result, values.as_slice(), ops.as_slice()) {
                p1 += result as usize;
                break;
            }
        }
        for ops in repeat_n(
            [Operand::Add, Operand::Mul, Operand::Concat].iter(),
            values.len() - 1,
        )
        .multi_cartesian_product()
        {
            if check_equation(result, values.as_slice(), ops.as_slice()) {
                p2 += result as usize;
                break;
            }
        }
    }
    Ok((p1.to_string(), p2.to_string()))
}
