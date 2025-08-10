use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Clone, Copy, Debug, Default)]
struct Node {
    zero: u32, // Index to 0 node
    one: u32,  // Index to 1 node
    count: usize,
}

#[derive(Debug)]
struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        let root = Node::default();
        let mut nodes = Vec::with_capacity(1024);
        nodes.push(root);

        Trie { nodes }
    }

    fn insert_node(&mut self) -> u32 {
        self.nodes.push(Node::default());
        (self.nodes.len() - 1) as u32
    }

    fn add(&mut self, value: u32) {
        let mut current_idx = 0;
        self.nodes[current_idx].count += 1;
        for i in (0..32).rev() {
            let bit = (value >> i) & 1;

            let next_idx = if bit == 0 {
                if self.nodes[current_idx].zero == 0 {
                    // Create new node
                    let new_node_idx = self.insert_node();
                    self.nodes[current_idx].zero = new_node_idx;
                }
                self.nodes[current_idx].zero
            } else {
                if self.nodes[current_idx].one == 0 {
                    // Create new node
                    let new_node_idx = self.insert_node();
                    self.nodes[current_idx].one = new_node_idx;
                }
                self.nodes[current_idx].one
            };

            current_idx = next_idx as usize;
            self.nodes[current_idx].count += 1;
        }
    }

    fn delete(&mut self, value: u32) {
        let mut current_idx = 0;
        self.nodes[current_idx].count -= 1;
        for i in (0..32).rev() {
            let bit = (value >> i) & 1;
            let next_idx = if bit == 0 {
                self.nodes[current_idx].zero
            } else {
                self.nodes[current_idx].one
            };
            current_idx = next_idx as usize;
            self.nodes[current_idx].count -= 1;
        }
    }

    fn query(&mut self, value: u32) {
        let mut max_xor: u32 = 0;
        let mut current_idx = 0;
        for i in (0..32).rev() {
            let bit = (value >> i) & 1;
            if bit == 0 {
                // Try to go to the 1-bit child
                if self.nodes[current_idx].one != 0
                    && self.nodes[self.nodes[current_idx].one as usize].count > 0
                {
                    max_xor |= 1 << i;
                    current_idx = self.nodes[current_idx].one as usize;
                } else {
                    // Must go to the 0-bit child
                    current_idx = self.nodes[current_idx].zero as usize;
                }
            } else {
                // bit == 1
                // Try to go to the 0-bit child
                if self.nodes[current_idx].zero != 0
                    && self.nodes[self.nodes[current_idx].zero as usize].count > 0
                {
                    max_xor |= 1 << i;
                    current_idx = self.nodes[current_idx].zero as usize;
                } else {
                    current_idx = self.nodes[current_idx].one as usize;
                }
            }
        }
        println!("{max_xor}");
    }
}

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
    let n_ops = next_num(&mut substr_iter);

    let mut trie = Trie::new();

    let mut parts: Vec<&str>;
    trie.add(0);
    for _ in 0..n_ops {
        input.clear();
        reader.read_line(&mut input).unwrap();
        parts = input.split_ascii_whitespace().collect();

        let value = parts[1].parse().unwrap();
        match parts[0] {
            "+" => trie.add(value),
            "-" => trie.delete(value),
            "?" => trie.query(value),
            _ => unreachable!(),
        };
    }
}
