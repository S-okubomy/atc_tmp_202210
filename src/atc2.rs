use itertools::Itertools;
use proconio:: { input };
use std::collections::{ VecDeque, HashMap, HashSet };
use std::fmt::format;

fn main() {
    input! {
        n: usize,
        mut p_vec: [usize; n],
    }
    let mut j = n -2;
    while p_vec[j] < p_vec[j+1] {
        j -= 1;
    }

    let mut k = n -1;
    while p_vec[j] < p_vec[k] {
        k -= 1;
    }
    p_vec.swap(j, k);
    // println!("{:?}", p_vec);
    p_vec[j+1..].reverse();
    // println!("{:?}", p_vec);
    println!("{}", p_vec.iter().join(" "));
}