use std::io::{self, BufRead, Read, Write};

fn main() {
    let mut stdin = io::stdin();
    let mut lock = stdin.lock();
    let mut buffer = String::new();

    lock.read_line(&mut buffer).unwrap();
    let t: u32 = buffer.trim_end().parse().unwrap();

    println!("{t}");

    for _ in 0..t {
        buffer.clear();
        lock.read_line(&mut buffer).unwrap();

        buffer.clear();
        lock.read_line(&mut buffer).unwrap();
        let heaps: Vec<u32> = buffer
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap()) // Parse each string part into a u32.
            .collect();

        let nim_sum = heaps.iter().fold(0, |acc, &x| acc ^ x);

        if nim_sum != 0 {
            println!("first");
        } else {
            println!("second");
        }
    }
}
