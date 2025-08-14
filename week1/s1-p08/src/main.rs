use std::{
    io::{self, BufRead, Read},
    process::exit,
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    reader.read_to_string(&mut input).unwrap();

    let string = input.trim_end().as_bytes();

    let len = string.len();

    let mut mismatched_pair = false;
    for i in 0..len / 2 {
        if string[i] != (string[len - i - 1]) {
            if mismatched_pair {
                println!("NO");
                exit(0);
            } else {
                mismatched_pair = true;
            }
        }
    }

    if mismatched_pair {
        println!("YES");
    } else if len % 2 == 1 {
        println!("YES");
    } else {
        println!("NO");
    }
}
