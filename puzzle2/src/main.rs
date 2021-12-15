use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"^([a-z]+)\s(\d+)$").expect("Invalid regex");
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    for line in stdin.lock().lines() {
        let line_str = line.expect("No line?");
        let capture = re.captures(&line_str).expect("No capture");
        let incr = capture
            .get(2)
            .expect("No 2nd group")
            .as_str()
            .parse::<i32>()
            .expect("Invalid int");
        match capture.get(1).expect("No 1st group").as_str() {
            "forward" => {
                horizontal += incr;
                depth += aim * incr;
            }
            "down" => {
                aim += incr;
            }
            "up" => {
                aim -= incr;
            }
            _ => {
                panic!("Unknow direction")
            }
        }
    }
    println!(
        "Horizontal {}, Depth {}, result {}",
        horizontal,
        depth,
        horizontal * depth
    );
}
