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
fn count_subarrays(n: usize, t: &[i64], max_force: i64) -> u64 {
    let mut total_count: u64 = 0;

    // Arrays para guardar os índices do primeiro elemento menor à esquerda (L)
    // e à direita (R). Usamos -1 e N como sentinelas.
    let mut l = vec![-1; n];
    let mut stack = Vec::new();
    for i in 0..n {
        while !stack.is_empty() && t[*stack.last().unwrap()] >= t[i] {
            stack.pop();
        }
        if let Some(&last_idx) = stack.last() {
            l[i] = last_idx as i64;
        }
        stack.push(i);
    }

    let mut r = vec![n as i64; n];
    let mut stack = Vec::new();
    for i in (0..n).rev() {
        while !stack.is_empty() && t[*stack.last().unwrap()] > t[i] {
            stack.pop();
        }
        if let Some(&last_idx) = stack.last() {
            r[i] = last_idx as i64;
        }
        stack.push(i);
    }

    // Itera por cada boneca para contagem
    for i in 0..n {
        let ti = t[i];
        if ti == 0 {
            continue;
        }

        // Calcula o comprimento máximo do subvetor
        let max_len_f64 = (max_force as f64) / (ti as f64);
        let max_len_sqrt = max_len_f64.sqrt();
        let max_len = max_len_sqrt.floor() as i64;

        if max_len == 0 {
            continue;
        }

        let len_left = i as i64 - l[i];
        let len_right = r[i] - i as i64;

        // Contagem de pares (i, j) onde (i+j+1) <= max_len.
        // l é o deslocamento para a esquerda (1 a len_left), r é para a direita (1 a len_right).
        // A contagem é o número de pares (l, r) tal que l+r <= max_len e l, r estão dentro dos limites.

        let mut count_for_i = 0;

        // Se o lado esquerdo é menor, iterar sobre ele
        if len_left <= len_right {
            for l_offset in 1..=len_left {
                let r_offset_max = max_len - l_offset;
                count_for_i += r_offset_max.min(len_right);
            }
        } else {
            // Se o lado direito é menor, iterar sobre ele
            for r_offset in 1..=len_right {
                let l_offset_max = max_len - r_offset;
                count_for_i += l_offset_max.min(len_left);
            }
        }

        total_count += count_for_i as u64;
    }

    total_count
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let (n, k): (u64, u64) = read_two_numbers(&mut reader);
    let mut dolls: Vec<i64> = read_n_numbers(&mut reader, n as usize);

    let mut low = 1i64;
    let mut high = 10i64.pow(17);
    let mut ans = high;

    while low <= high {
        let mid: i64 = low + (high - low) / 2;
        let count = count_subarrays(n as usize, &dolls, mid);

        if count >= k {
            ans = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    println!("{}", ans);
}
