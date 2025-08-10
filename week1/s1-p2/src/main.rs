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
    let len = next_num(&mut substr_iter);

    let mut input = String::new();
    let mut numbers: Vec<i32> = Vec::with_capacity(len);

    reader.read_line(&mut input).unwrap();

    let mut substr_iter = input.split_ascii_whitespace();
    for _ in 0..len {
        numbers.push(next_num(&mut substr_iter) as i32)
    }

    let mut freq_map = HashMap::new();

    let mut count: usize = 0;
    // j - i  == Ai + Aj
    // lh     == rh
    // j - Aj == i + Ai
    for i in 0..len {
        let lh = (i as i32) + 1 - numbers[i];
        //println!("{} {} {target}", i + 1, numbers[i]);

        if let Some(value) = freq_map.get_mut(&lh) {
            count += *value;
        }

        let rh = (i as i32) + 1 + numbers[i];

        if let Some(value) = freq_map.get_mut(&rh) {
            *value += 1;
        } else {
            freq_map.insert(rh, 1);
        }
    }

    println!("{count}");
}
