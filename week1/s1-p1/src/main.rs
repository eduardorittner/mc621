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
    let n = next_num(&mut substr_iter);
    let k = next_num(&mut substr_iter);

    let mut vec_input = String::new();
    let mut prime_input = Vec::with_capacity(k);

    reader.read_line(&mut vec_input).unwrap();

    let mut substr_iter = vec_input.split_ascii_whitespace();
    for _ in 0..k {
        prime_input.push(next_num(&mut substr_iter))
    }

    let primes: &[usize] = &prime_input;

    let mut total_count: usize = 0;

    for i in 1..(1 << k) {
        let mut product: usize = 1;
        let mut set_bits = 0;

        for j in 0..k {
            if (i >> j) & 1 == 1 {
                set_bits += 1;

                if product > n / primes[j] {
                    product = n + 1;
                    break;
                }
                product *= primes[j];
            }
        }

        if product <= n {
            let count_for_subset = n / product;
            if set_bits % 2 == 1 {
                total_count += count_for_subset;
            } else {
                total_count -= count_for_subset;
            }
        }
    }

    println!("{total_count}");
}
