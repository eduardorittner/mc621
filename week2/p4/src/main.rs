use std::cmp;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
    }

    let min_depth = (n as f64).log2().floor() as u64;

    let two_pow_k_plus_1 = 2u64.pow((min_depth + 1) as u32);
    let fish_min_depth = two_pow_k_plus_1 - n;
    let fish_max_depth = (2 * n) - two_pow_k_plus_1;

    let total_questions = (fish_min_depth * min_depth) + (fish_max_depth * (min_depth + 1));

    let expected_questions = total_questions as f64 / n as f64;

    println!("{:.6}", expected_questions + 1.);
}
