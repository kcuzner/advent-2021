use std::io::{self, BufRead};

struct Lanternfish {
    counter: i32,
    replicas: i64,
}

impl Lanternfish {
    fn new(counter: i32, replicas: i64) -> Self {
        Self {
            counter: counter,
            replicas: replicas,
        }
    }

    /// Return true if a new lanternfish should spawn
    fn day(&mut self) -> bool {
        if self.counter == 0 {
            self.counter = 6;
            true
        } else {
            self.counter -= 1;
            false
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let init_state = stdin
        .lock()
        .lines()
        .next()
        .expect("No first line?")
        .expect("No string?");

    let mut fish: Vec<_> = init_state
        .split(",")
        .map(|s| Lanternfish::new(s.parse::<i32>().expect("Invalid number"), 1))
        .collect::<Vec<_>>();

    for _ in 0..256 {
        let mut replicas = 0;
        for f in fish.iter_mut() {
            if f.day() {
                replicas += f.replicas;
            }
        }
        fish.push(Lanternfish::new(8, replicas));
    }
    let total: i64 = fish.iter().map(|f| f.replicas).sum();
    println!("Fish: {}", total);
}
