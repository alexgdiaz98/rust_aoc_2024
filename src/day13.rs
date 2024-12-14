use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;

use crate::matrix::Matrix2x2;

fn read_line(re: &regex::Regex, line: &str) -> (i64, i64) {
    let line_0 = re.captures(line).unwrap();
    let a = line_0.get(1).unwrap().as_str().parse().unwrap();
    let b = line_0.get(2).unwrap().as_str().parse().unwrap();
    (a, b)
}

/// See: https://www.youtube.com/watch?v=jBsC34PxzoM
fn is_in_linear_space(mat_a: Matrix2x2, i: i64, j: i64) -> Option<usize> {
    let det_a = mat_a.det();
    let det_ax = Matrix2x2::new(i, mat_a.b, j, mat_a.d).det();
    let det_ay = Matrix2x2::new(mat_a.a, i, mat_a.c, j).det();
    if det_ax % det_a == 0 && det_ay % det_a == 0 {
        let x = det_ax / det_a;
        let y = det_ay / det_a;
        return Some(3 * x as usize + y as usize);
    }
    None
}

pub fn day13(input_path: &Path) -> Result<(String, String)> {
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let regex = regex::Regex::new(r".+X.(\d+), Y.(\d+)$").unwrap();
    for machine in contents.split("\n\n") {
        let mut machine_lines = machine.split('\n');
        let (a, c) = read_line(&regex, machine_lines.next().unwrap());
        let (b, d) = read_line(&regex, machine_lines.next().unwrap());
        let (i, j) = read_line(&regex, machine_lines.next().unwrap());
        let mat_a = Matrix2x2::new(a, b, c, d);
        if let Some(addend) = is_in_linear_space(mat_a, i, j) {
            p1 += addend;
        }
        if let Some(addend) = is_in_linear_space(mat_a, i + 10000000000000, j + 10000000000000) {
            p2 += addend;
        }
    }
    Ok((p1.to_string(), p2.to_string()))
}
