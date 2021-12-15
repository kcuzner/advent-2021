use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut first = true;
    let mut increased = 0;
    let mut last = 0;
    for line in stdin.lock().lines() {
        let value = line
            .expect("No line?")
            .parse::<u32>()
            .expect("Invalid integer");
        if value > last && !first {
            increased += 1;
        }
        last = value;
        first = false;
    }
    println!("Increased {} times", increased);
}
