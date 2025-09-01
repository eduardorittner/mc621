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

    let n: usize = read_one_number(&mut reader);
    let days: Vec<usize> = read_n_numbers(&mut reader, n);

    let mut max_streak: usize = 1;
    let mut current_streak: usize = 1;
    let mut last = days[0];

    for day in days {
        if day == last + 1 {
            current_streak += 1;
            if current_streak > max_streak {
                max_streak = current_streak;
            }
        } else {
            current_streak = 1;
        }
        last = day;
    }

    println!("{max_streak}");
}
