use std::fmt;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
pub enum Op {
    Add(i64),
    Mul(i64),
}

#[derive(Clone, Copy)]
pub struct GatePair {
    pub left: Op,
    pub right: Op,
}

pub fn parse_gate_pair(line: &str) -> GatePair {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 4 {
        unreachable!();
    }
    let parse_one_op = |op_str: &str, val_str: &str| -> Op {
        let val = val_str.parse::<i64>().unwrap();
        match op_str {
            "+" => Op::Add(val),
            "x" => Op::Mul(val),
            _ => unreachable!(),
        }
    };
    let left = parse_one_op(parts[0], parts[1]);
    let right = parse_one_op(parts[2], parts[3]);
    GatePair { left, right }
}

pub fn parse_all_gates<'a, I>(lines: I) -> Vec<GatePair>
where
    I: Iterator<Item = &'a str>,
{
    lines
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_gate_pair)
        .collect()
}

pub fn solve(gate_pairs: &[GatePair]) -> i64 {
    let mut states: Vec<(i64, i64)> = vec![(1, 1)];
    let mut next_states_temp = Vec::new();
    for gate_pair in gate_pairs {
        next_states_temp.clear();

        for &(l, r) in &states {
            let delta_l = match gate_pair.left {
                Op::Add(a) => a,
                Op::Mul(a) => l * (a - 1),
            };
            let delta_r = match gate_pair.right {
                Op::Add(a) => a,
                Op::Mul(a) => r * (a - 1),
            };

            next_states_temp.push((l + delta_l + delta_r, r));
            next_states_temp.push((l, r + delta_l + delta_r));
        }

        next_states_temp.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

        let mut pruned_states = Vec::new();
        let mut last_kept_state = next_states_temp[0];
        if next_states_temp.is_empty() {
            states = pruned_states;
            continue;
        }
        pruned_states.push(next_states_temp[0]);
        for &current_state in &next_states_temp[1..] {
            if current_state.0 > last_kept_state.0 && current_state.1 < last_kept_state.1 {
                pruned_states.push(current_state);
                last_kept_state = current_state;
            }
        }
        states = pruned_states;
    }
    states.iter().map(|(l, r)| l + r).max().unwrap_or(0)
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut line_iterator = lines.iter();

    let num_test_cases: usize = match line_iterator.next() {
        Some(line) => line.parse().unwrap(),
        None => return, // No input, do nothing.
    };

    for _ in 0..num_test_cases {
        let num_gates: usize = match line_iterator.next() {
            Some(line) => line.parse().unwrap(),
            None => {
                unreachable!()
            }
        };

        let gate_lines = line_iterator.by_ref().take(num_gates).map(|s| s.as_str());

        let gate_pairs = parse_all_gates(gate_lines);
        let max_people = solve(&gate_pairs);
        println!("{}", max_people);
    }
}
