use std::io::{self, BufRead};

type Matrix = Vec<Vec<u64>>;

const MODULO: u64 = 1_000_000_000;

fn multiply_matrices(a: &Matrix, b: &Matrix, k: usize) -> Matrix {
    let mut result = vec![vec![0; k]; k];
    for i in 0..k {
        for j in 0..k {
            for l in 0..k {
                result[i][j] = (result[i][j] + a[i][l] * b[l][j]) % MODULO;
            }
        }
    }
    result
}

fn matrix_pow(base: &Matrix, exp: u64, k: usize) -> Matrix {
    let mut res = vec![vec![0; k]; k];
    for i in 0..k {
        res[i][i] = 1;
    }

    let mut base = base.clone();
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            res = multiply_matrices(&res, &base, k);
        }
        base = multiply_matrices(&base, &base, k);
        exp /= 2;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let cases: u32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..cases {
        let elements: usize = lines.next().unwrap().parse().unwrap();
        let b: Vec<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let c: Vec<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let n: u64 = lines.next().unwrap().trim().parse().unwrap();

        if n <= elements as u64 {
            println!("{}", b[(n - 1) as usize]);
            continue;
        }

        let mut t = vec![vec![0; elements]; elements];
        for i in 0..elements {
            t[0][i] = c[i];
        }
        for i in 1..elements {
            t[i][i - 1] = 1;
        }

        let t_pow = matrix_pow(&t, n - elements as u64, elements);

        let mut an = 0;
        for j in 0..elements {
            an = (an + t_pow[0][j] * b[elements - 1 - j]) % MODULO;
        }
        println!("{}", an);
    }
}
