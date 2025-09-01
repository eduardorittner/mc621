use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_009;

type Matrix = [[i64; 3]; 3];

fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

fn matrix_pow(mut m: Matrix, mut exp: u64) -> Matrix {
    let mut res = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]; // Identity matrix
    while exp > 0 {
        if exp % 2 == 1 {
            res = matrix_multiply(&res, &m);
        }
        m = matrix_multiply(&m, &m);
        exp /= 2;
    }
    res
}

fn solve() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let n: u64 = line.unwrap().trim().parse().unwrap();

        if n == 0 {
            break;
        }

        if n == 1 {
            println!("0");
            continue;
        }
        if n == 2 {
            println!("1");
            continue;
        }
        if n == 3 {
            println!("2");
            continue;
        }

        let m = [[1, 1, 1], [1, 0, 0], [0, 1, 0]];
        let power = n - 3;
        let m_pow = matrix_pow(m, power);

        let initial_vector = [2, 1, 0];

        let result = (m_pow[0][0] * initial_vector[0]
            + m_pow[0][1] * initial_vector[1]
            + m_pow[0][2] * initial_vector[2])
            % MOD;

        println!("{}", result);
    }
}

fn main() {
    solve();
}
