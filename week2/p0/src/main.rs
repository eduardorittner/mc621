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
    let input: Vec<usize> = read_n_numbers(&mut reader, 5);

    let total: usize = input.iter().sum();
    let half = total / 2;

    let mut winners = Vec::new();

    if input[0] + input[4] > half {
        winners.push("Rafael");
    }
    if input[1] + input[4] > half {
        winners.push("Leonardo");
    }
    if input[2] + input[4] > half {
        winners.push("Donatello");
    }
    if input[3] + input[4] > half {
        winners.push("Michelangelo");
    }

    if winners.len() == 0 {
        println!("sem vencedores");
    } else {
        winners.sort();
        for w in winners {
            println!("{w}");
        }
    }
}
