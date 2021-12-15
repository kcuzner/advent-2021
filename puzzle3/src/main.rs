use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().collect::<Result<_, _>>().unwrap();
    let size = lines[0].len();
    let gamma: i32 = (0..size)
        .map(|i| {
            let (zeros, ones) = lines.iter().fold((0, 0), |(zeros, ones), s| {
                let index = size - i - 1;
                if s.chars().nth(index).expect("No number?") == '1' {
                    (zeros, ones + 1)
                } else {
                    (zeros + 1, ones)
                }
            });
            println!("Pos {}, {} {}", i, zeros, ones);
            if zeros > ones {
                0
            } else {
                2_i32.pow(i as u32)
            }
        })
        .sum();
    let mask = 2_i32.pow(size as u32)-1;
    let epsilon = (!gamma) & mask;
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}
