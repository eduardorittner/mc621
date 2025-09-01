use std::{
    collections::{BTreeSet, HashSet},
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

fn read_set(stdin: &mut StdinLock) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    let mut input = String::new();
    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let line = input.trim();

        if line == "*" {
            return set;
        }
        set.insert(line.to_string());
    }
}

fn read_pairs(stdin: &mut StdinLock) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let line = input.trim();

        if line == "" {
            return pairs;
        }

        let (lh, rh) = line.split_once(" ").unwrap();

        pairs.push((lh.to_string(), rh.to_string()));
    }
}

fn first_mismatched_char(left: &str, right: &str) -> Option<usize> {
    left.chars()
        .into_iter()
        .zip(right.chars().into_iter())
        .position(|(c1, c2)| c1 != c2)


fn process_pair(set: &BTreeSet<String>, pair: (String, String)) {
    let left = pair.0;
    let right = pair.1;
    let mut steps = 0;

    loop {
        if let Some(c) = first_mismatched_char(&left, &right) {}
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let tests: usize = read_one_number(&mut reader);

    for _ in 0..tests {
        let set = read_set(&mut reader);
        let pairs = read_pairs(&mut reader);

        for pair in pairs {}
    }
}
