use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let mut substr_iter = input.split_ascii_whitespace();
    let mut next_num = |substr_iter: &mut std::str::SplitAsciiWhitespace<'_>| -> usize {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };
    let test_cases = next_num(&mut substr_iter);

    for _ in 0..test_cases {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let cards: usize = input.trim_end().parse().unwrap();

        let mut alice = Vec::with_capacity(cards);
        let mut bob = Vec::with_capacity(cards);

        input.clear();
        reader.read_line(&mut input).unwrap();

        for (index, c) in input.trim_end().chars().enumerate() {
            match c {
                'A' => {
                    alice.push(index + 1);
                }
                'B' => {
                    bob.push(index + 1);
                }
                _ => unreachable!(),
            };
        }

        solve(cards, alice, bob);
    }
}

fn solve(n: usize, alice: Vec<usize>, bob: Vec<usize>) {
    let bob_max_card = bob.last().unwrap();
    let bob_min_card = bob[0];

    // BABA
    for card in alice.iter().rev() {
        if *card == n {
            if bob[0] != 1 {
                println!("Alice");
                return;
            }
        } else if *card == 1 {
            if bob_min_card == n {
                println!("Alice");
                return;
            }
        } else if card > bob_max_card {
            println!("Alice");
            return;
        }
    }
    println!("Bob");
}
