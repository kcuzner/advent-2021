use std::io::{self, BufRead};

struct Lanternfish {
    counter: i32,
}

impl Lanternfish {
    fn new(counter: i32) -> Self {
        Self { counter: counter }
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
        .map(|s| Lanternfish::new(s.parse::<i32>().expect("Invalid number")))
        .collect::<Vec<_>>();
    for _ in 0..80 {
        let mut to_add: Vec<_> = Vec::<Lanternfish>::new();
        for f in fish.iter_mut() {
            if f.day() {
                to_add.push(Lanternfish::new(8));
            }
        }
        fish.extend(to_add);
    }
    println!("Fish: {}", fish.len());
}
