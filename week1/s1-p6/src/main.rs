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

enum Ops {
    Inc { lh: usize, rh: usize, inc: i64 },
    Min { lh: usize, rh: usize },
}

fn read_ops(stdin: &mut StdinLock) -> Ops {
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let numbers: Vec<_> = input.trim_end().split_ascii_whitespace().collect();
    if numbers.len() == 2 {
        let (lh, rh) = (numbers[0].parse().unwrap(), numbers[1].parse().unwrap());
        Ops::Min { lh, rh }
    } else {
        let (lh, rh, inc) = (
            numbers[0].parse().unwrap(),
            numbers[1].parse().unwrap(),
            numbers[2].parse().unwrap(),
        );
        Ops::Inc { lh, rh, inc }
    }
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

#[derive(Clone, Copy)]
struct Node {
    min: i64,
    lazy: i64,
}

struct SegmentTree {
    n: usize,
    tree: Vec<Node>,
}

impl SegmentTree {
    fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut tree = vec![Node { min: 0, lazy: 0 }; 4 * n];
        let mut seg_tree = SegmentTree { n, tree };
        seg_tree.build(arr, 0, 0, n - 1);
        seg_tree
    }

    fn build(&mut self, arr: &[i64], idx: usize, l: usize, r: usize) {
        if l == r {
            self.tree[idx].min = arr[l];
            return;
        }
        let mid = (l + r) / 2;
        self.build(arr, 2 * idx + 1, l, mid);
        self.build(arr, 2 * idx + 2, mid + 1, r);
        self.tree[idx].min = self.tree[2 * idx + 1].min.min(self.tree[2 * idx + 2].min);
    }

    fn push(&mut self, idx: usize, l: usize, r: usize) {
        if self.tree[idx].lazy != 0 {
            self.tree[idx].min += self.tree[idx].lazy;
            if l != r {
                self.tree[2 * idx + 1].lazy += self.tree[idx].lazy;
                self.tree[2 * idx + 2].lazy += self.tree[idx].lazy;
            }
            self.tree[idx].lazy = 0;
        }
    }

    fn update_range(&mut self, idx: usize, l: usize, r: usize, seg_l: usize, seg_r: usize, v: i64) {
        self.push(idx, l, r);
        if seg_r < l || seg_l > r {
            return;
        }
        if seg_l <= l && r <= seg_r {
            self.tree[idx].lazy += v;
            self.push(idx, l, r);
            return;
        }
        let mid = (l + r) / 2;
        self.update_range(2 * idx + 1, l, mid, seg_l, seg_r, v);
        self.update_range(2 * idx + 2, mid + 1, r, seg_l, seg_r, v);
        self.tree[idx].min = self.tree[2 * idx + 1].min.min(self.tree[2 * idx + 2].min);
    }

    fn query_range(&mut self, idx: usize, l: usize, r: usize, seg_l: usize, seg_r: usize) -> i64 {
        self.push(idx, l, r);
        if seg_r < l || seg_l > r {
            return i64::MAX;
        }
        if seg_l <= l && r <= seg_r {
            return self.tree[idx].min;
        }
        let mid = (l + r) / 2;
        let left_min = self.query_range(2 * idx + 1, l, mid, seg_l, seg_r);
        let right_min = self.query_range(2 * idx + 2, mid + 1, r, seg_l, seg_r);
        left_min.min(right_min)
    }

    pub fn update(&mut self, l: usize, r: usize, v: i64) {
        self.update_range(0, 0, self.n - 1, l, r, v);
    }

    pub fn query(&mut self, l: usize, r: usize) -> i64 {
        self.query_range(0, 0, self.n - 1, l, r)
    }
}

struct CircularArray {
    seg_tree: SegmentTree,
    n: usize,
}

impl CircularArray {
    pub fn new(arr: &[i64]) -> Self {
        CircularArray {
            seg_tree: SegmentTree::new(arr),
            n: arr.len(),
        }
    }

    pub fn inc(&mut self, l: usize, r: usize, v: i64) {
        if l <= r {
            self.seg_tree.update(l, r, v);
        } else {
            self.seg_tree.update(l, self.n - 1, v);
            self.seg_tree.update(0, r, v);
        }
    }

    pub fn rmq(&mut self, l: usize, r: usize) -> i64 {
        if l <= r {
            self.seg_tree.query(l, r)
        } else {
            let left = self.seg_tree.query(l, self.n - 1);
            let right = self.seg_tree.query(0, r);
            left.min(right)
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n: usize = read_one_number(&mut reader);

    let numbers: Vec<usize> = read_n_numbers(&mut reader, n);

    let ops: usize = read_one_number(&mut reader);

    for _ in 0..ops {
        let op = read_ops(&mut stdin);
    }
}
