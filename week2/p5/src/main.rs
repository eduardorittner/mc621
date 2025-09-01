use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TukTuk {
    id: i32,
    score: i64,
}

impl PartialOrd for TukTuk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for TukTuk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let parts: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0];
    let q = parts[1];

    let mut tuk_tuks: Vec<TukTuk> = Vec::with_capacity(n as usize);
    let mut id_to_score: HashMap<i32, i64> = HashMap::new();

    for i in 1..=n {
        let score = (10i64.pow(9)) - ((i - 1) as i64 * 200000);
        let tuk_tuk = TukTuk { id: i, score };
        tuk_tuks.push(tuk_tuk.clone());
        id_to_score.insert(i, score);
    }

    let mut initial_scores = Vec::with_capacity(n as usize);
    for i in 1..=n {
        initial_scores.push(id_to_score.get(&i).unwrap());
    }
    for (i, score) in initial_scores.iter().enumerate() {
        print!("{}", score);
        if i < initial_scores.len() - 1 {
            print!(" ");
        }
    }
    println!();

    for _ in 0..q {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let p = parts[0];
        let x = parts[1];

        let old_score_p = *id_to_score.get(&p).unwrap();

        let new_score: i64;
        if x == 1 {
            let score_below = tuk_tuks[0].score;
            new_score = score_below + 1;
        } else {
            let score_above = tuk_tuks[x as usize - 2].score;
            let score_below = tuk_tuks[x as usize - 1].score;
            new_score = (score_above + score_below) / 2;
        }

        let points_gained = new_score - old_score_p;
        println!("{}", points_gained);

        id_to_score.insert(p, new_score);

        for tuk_tuk in tuk_tuks.iter_mut() {
            if tuk_tuk.id == p {
                tuk_tuk.score = new_score;
                break;
            }
        }

        tuk_tuks.sort_by(|a, b| b.score.cmp(&a.score));
    }
}
