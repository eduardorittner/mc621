use std::{
    fmt::Debug,
    io::{self, BufRead, StdinLock},
    str::FromStr,
};

fn read_one_number<T: FromStr<Err: Debug>>(stdin: &mut StdinLock) -> T {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input.trim_end().parse().unwrap()
}

fn read_n_numbers<T: FromStr<Err: Debug>>(stdin: &mut StdinLock, n: usize) -> Vec<T> {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
        .trim_end()
        .split_ascii_whitespace()
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let n = read_one_number(&mut reader);
    let mut chars = String::new();
    reader.read_line(&mut chars);
    chars.trim_end();

    let chars: Vec<char> = chars.chars().collect();

    let mut balance = 0;
    let mut min_balance = 0;
    let mut rotation_index = 0;

    for i in 0..n {
        if chars[i] == '(' {
            balance += 1;
        } else {
            balance -= 1;
        }

        if balance < min_balance {
            min_balance = balance;
            rotation_index = i + 1;
        }
    }
    println!("{}", rotation_index);
}
