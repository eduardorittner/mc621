use std::collections::HashMap;
use std::io::{self, BufRead};

fn find_longest_path(
    current_state: String,
    memo: &mut HashMap<String, Option<Vec<String>>>,
    visiting_stack: &mut Vec<String>,
) -> Option<Vec<String>> {
    if current_state.chars().all(|c| c == 'T') {
        return Some(vec![current_state]);
    }

    if visiting_stack.contains(&current_state) {
        return None; // Ciclo detectado
    }

    if let Some(result) = memo.get(&current_state) {
        return result.clone();
    }

    visiting_stack.push(current_state.clone());

    let n = current_state.len();
    let mut max_path: Option<Vec<String>> = None;

    for i in 0..n {
        if current_state.as_bytes()[i] == b'H' {
            let num_left_coins = i;
            let num_combinations = 1 << num_left_coins;

            for j in 0..num_combinations {
                let mut new_state_chars: Vec<char> = current_state.chars().collect();
                new_state_chars[i] = 'T';

                for k in 0..num_left_coins {
                    if (j >> k) & 1 == 1 {
                        new_state_chars[k] = if new_state_chars[k] == 'H' { 'T' } else { 'H' };
                    }
                }

                let new_state = new_state_chars.iter().collect::<String>();

                if let Some(mut path) = find_longest_path(new_state, memo, visiting_stack) {
                    path.insert(0, current_state.clone());
                    if max_path.is_none() || path.len() > max_path.as_ref().unwrap().len() {
                        max_path = Some(path);
                    }
                }
            }
        }
    }

    visiting_stack.pop();

    memo.insert(current_state, max_path.clone());
    max_path
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut initial_state = String::new();
    handle.read_line(&mut initial_state).unwrap();
    let initial_state = initial_state.trim().to_string();

    let mut memo: HashMap<String, Option<Vec<String>>> = HashMap::new();
    let mut visiting_stack: Vec<String> = Vec::new();

    let result = find_longest_path(initial_state, &mut memo, &mut visiting_stack);

    match result {
        Some(path) => {
            println!("{}", path.len());
            for state in path {
                println!("{}", state);
            }
        }
        None => {
            println!("-1");
        }
    }
}
