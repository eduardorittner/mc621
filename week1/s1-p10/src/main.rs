use std::io::{self, BufRead};

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
    let operations = next_num(&mut substr_iter);

    let mut input = String::new();
    let mut a = Vec::with_capacity(len);

    reader.read_line(&mut input).unwrap();

    let mut substr_iter = input.split_ascii_whitespace();
    for _ in 0..len {
        a.push(next_num(&mut substr_iter))
    }

    let mut input = String::new();
    let mut b = Vec::with_capacity(len);

    reader.read_line(&mut input).unwrap();

    let mut substr_iter = input.split_ascii_whitespace();
    for _ in 0..operations {
        b.push(next_num(&mut substr_iter))
    }

    for i in 0..operations {
        let to_remove = b[i];

        for j in 0..a.len() {
            if a[j] == to_remove {
                a.remove(j);
                break;
            }
        }
    }

    for n in a {
        print!("{n} ");
    }
    println!();
}
