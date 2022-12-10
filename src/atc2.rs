use itertools::Itertools;
use proconio:: { input };
use std::collections::{ VecDeque };

fn main() {
    input! {
        n: usize, k: usize,
        a_vec: [usize; n],
    }
    let mut deque: VecDeque<usize> = VecDeque::from(a_vec);
    for _ in 0..k {
        deque.pop_front();
        deque.push_back(0);
    }
    println!("{}", deque.iter().map(|x| x.to_string()).join(" "));
}