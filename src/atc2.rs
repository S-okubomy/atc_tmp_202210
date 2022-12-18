use std::collections::HashSet;

use itertools::Itertools;
use proconio:: { input };

fn main() {
    input! {
        s: String,
        t: String,
    }
    let c_s_vec: Vec<char> = s.chars().collect();
    let c_t_vec: Vec<char> = t.chars().collect();

    let len = c_s_vec.len();
    for i in 0..len {
        if c_s_vec[i] != c_t_vec[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", c_t_vec.len());
}