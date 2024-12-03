use std::fs::read_to_string;
use std::path::Path;

#[allow(dead_code)]
pub fn day03(input_path: &Path) -> (String, String) {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut enabled = true;
    let re = regex::Regex::new(r"(?:do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    for line in contents.split("\n") {
        for re_match in re.captures_iter(line) {
            match re_match.get(0).unwrap().as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    p1 += re_match.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        * re_match.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    if enabled {
                        p2 += re_match.get(1).unwrap().as_str().parse::<usize>().unwrap()
                            * re_match.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    }
                }
            }
        }
    }
    (p1.to_string(), p2.to_string())
}
