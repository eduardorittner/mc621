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

fn read_two_numbers<T: FromStr<Err: Debug>>(stdin: &mut StdinLock) -> (T, T) {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let input: Vec<_> = input.trim_end().split_ascii_whitespace().collect();

    (input[0].parse().unwrap(), input[1].parse().unwrap())
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
    let pow: i64 = read_one_number(&mut reader);
    let mut numbers: Vec<i64> = read_n_numbers(&mut reader, n);

    let max: i64 = pow - 1;
    let min: i64 = -pow;

    let mut neg: Vec<_> = numbers.iter().filter(|n| **n < 0).collect();
    neg.sort();
    neg.reverse();

    let mut pos: Vec<_> = numbers.iter().filter(|n| **n >= 0).collect();
    pos.sort();

    let mut result = Vec::with_capacity(n);

    let mut pos_ptr = 0;
    let mut neg_ptr = 0;
    let mut sum: i64 = 0;
    let mut added = false;

    while pos_ptr < pos.len() || neg_ptr < neg.len() {
        added = false;
        if pos_ptr < pos.len() && sum + pos[pos_ptr] <= max {
            sum += pos[pos_ptr];
            result.push(pos[pos_ptr]);
            pos_ptr += 1;
            added = true;
        } else if neg_ptr < neg.len() && sum + neg[neg_ptr] >= min {
            sum += neg[neg_ptr];
            result.push(neg[neg_ptr]);
            neg_ptr += 1;
            added = true;
        }

        if !added {
            println!("N");
            return;
        }
    }

    println!("S");
    let result: Vec<String> = result.iter().map(|&x| x.to_string()).collect();
    println!("{}", result.join(" "));
}
