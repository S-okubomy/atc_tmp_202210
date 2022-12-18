use std::collections::HashSet;

use itertools::Itertools;
use proconio:: { input };

fn main() {
    input! {
        mut k: usize,
    }

    // TODO 修正
    let k_copy = k;
    let mut fact = 1;
    let lim: usize = (k as f64).sqrt() as usize;
    let mut fact_vec: Vec<(usize, usize)> = Vec::new();
    for x in 2..=lim {
        if k % x == 0 {
            let mut cnt = 0;
            while k % x == 0 {
                cnt += 1;
                k /= x;
            }
            fact_vec.push((x, cnt));
        }
    }

    let len = fact_vec.len();
    if len == 0 {
        println!("{}", k_copy);
    } else {
        println!("{:?}", fact_vec[len-1].0);
    }
}