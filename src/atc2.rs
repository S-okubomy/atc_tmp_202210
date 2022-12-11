use itertools::Itertools;
use proconio:: { input };
use std::collections::{ VecDeque, HashMap, HashSet };
use std::fmt::format;

fn main() {
    input! {
        n: usize,
        p_vec: [usize; n],
    }

    // TODO 修正
    let mut perm_vec: Vec<Vec<usize>> = Vec::new();
    for perm in p_vec.iter().permutations(n) {
        perm_vec.push(perm.into_iter().map(|x| *x).collect());
    }
    perm_vec.sort_by(|a, b| a.cmp(b));

    if let Some(trg_index) = perm_vec.iter().position(|v| { v.iter().join("") == p_vec.iter().join("") }) {
        println!("{}", perm_vec[trg_index-1].iter().join(" "));
    }
}