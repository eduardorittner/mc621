use std::io;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).unwrap();
    let mountains: Vec<i64> = a_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if mountains.len() != n {
        return;
    }

    let mut r1: i64 = 0;
    for i in 0..n {
        if i % 2 == 0 {
            r1 += mountains[i];
        } else {
            r1 -= mountains[i];
        }
    }

    let mut r_values: Vec<i64> = vec![0; n];
    r_values[0] = r1;

    for i in 0..(n - 1) {
        r_values[i + 1] = 2 * mountains[i] - r_values[i];
    }

    for i in 0..n {
        print!("{} ", r_values[i]);
    }
    println!();
}
