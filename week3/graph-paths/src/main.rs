use std::io::{self, BufRead};

fn multiply(a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>, n: usize) -> Vec<Vec<u64>> {
    let mut result = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for l in 0..n {
                result[i][j] += a[i][l] * b[l][j];
            }
        }
    }
    result
}

fn matrix_pow(mut mat: Vec<Vec<u64>>, mut k: u64, n: usize) -> Vec<Vec<u64>> {
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        result[i][i] = 1;
    }

    while k > 0 {
        if k % 2 == 1 {
            result = multiply(&result, &mat, n);
        }

        mat = multiply(&mat, &mat, n);

        k /= 2;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut parts = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());

    let n = parts.next().unwrap() as usize;
    let m = parts.next().unwrap();
    let k = parts.next().unwrap();

    let mut adj_matrix = vec![vec![0; n]; n];

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let edge_parts = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());

        let mut edge_iter = edge_parts;
        let a = edge_iter.next().unwrap();
        let b = edge_iter.next().unwrap();

        adj_matrix[a - 1][b - 1] = 1;
    }

    let result_matrix = matrix_pow(adj_matrix, k, n);

    println!("{}", result_matrix[0][n - 1]);
}
