use itertools::Itertools;
use proconio:: { input };
use std::{collections::{ VecDeque }, fmt::format};

fn main() {
    input! {
        n: usize, q: usize,
        tab_vec: [(usize, usize, usize); q],
    }

    // TODO 修正
    let mut n_vec: Vec<Vec<usize>> = vec![vec![]; n+1];
    for tab in tab_vec {
        let (t, a, b) = tab;
        if t == 1 {
            if !n_vec[a].contains(&b) {
                n_vec[a].push(b);
            }
        }

        if t == 2 {
            if let Some(remove_index) = n_vec[a].iter().position(|x| *x == b) {
                n_vec[a].remove(remove_index);
            }
        }

        if t == 3 {
            if n_vec.get(a).unwrap().contains(&b) && n_vec.get(b).unwrap().contains(&a) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}