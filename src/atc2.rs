use std::collections::HashSet;

use itertools::Itertools;
use proconio:: { input };

fn main() {
    input! {
        n: usize,
        s_vec: [isize; n],
    }

    let mut a_vec: Vec<isize> = Vec::new();
    a_vec.push(s_vec[0]);
    for i in 1..n {
        a_vec.push(s_vec[i] - s_vec[i - 1]);
    }
    println!("{}", a_vec.iter().join(" "));
}