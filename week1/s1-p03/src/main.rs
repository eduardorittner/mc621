use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    let mut current_unique = HashMap::new();

    reader.read_line(&mut input).unwrap();
    let mut substr_iter = input.split_ascii_whitespace();
    let mut next_num = |substr_iter: &mut std::str::SplitAsciiWhitespace<'_>| -> usize {
        substr_iter
            .next()
            .expect("Not enough input numbers")
            .parse()
            .expect("Input is not a number")
    };
    let size = next_num(&mut substr_iter);
    let window = next_num(&mut substr_iter);

    let mut vec_input = String::new();
    let mut numbers = Vec::with_capacity(size);

    reader.read_line(&mut vec_input).unwrap();

    let mut substr_iter = vec_input.split_ascii_whitespace();
    for _ in 0..size {
        numbers.push(next_num(&mut substr_iter))
    }

    // Populate initial window
    for i in 0..window {
        *current_unique.entry(numbers[i]).or_insert(0) += 1;
    }

    let mut unique_count = current_unique.keys().count();

    print!("{unique_count}");

    for i in 1..(size - window + 1) {
        let to_be_removed = numbers[i - 1];
        let to_be_added = numbers[i + window - 1];

        // Remove last window value
        if let Some(x) = current_unique.get_mut(&to_be_removed) {
            if *x == 1 {
                current_unique.remove(&to_be_removed);
                unique_count -= 1;
            } else {
                *x -= 1;
            }
        }

        // Insert first window value
        if let Some(x) = current_unique.get_mut(&to_be_added) {
            *x += 1;
        } else {
            current_unique.insert(to_be_added, 1);
            unique_count += 1;
        }

        print!(" {unique_count}");
    }

    println!();
}
