use itertools::Itertools;
use proconio:: { input };
use std::{collections::{ VecDeque }, fmt::format};

fn main() {
    input! {
        mut h: usize, mut m: usize,
    }

    while misjudge(h, m) {
        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
        }
        if h == 24 {
            h = 0;
        }
    }
    println!("{} {}", h, m);
}

fn misjudge(h: usize, m: usize) -> bool {
    let (a, b) = (h / 10, h % 10);
    let (c, d) = (m / 10, m % 10);
    let ac: usize = 10 * a + c;
    let bd: usize = 10 * b + d;
    is_out_24hours(ac, bd)
}

fn is_out_24hours(h: usize, m: usize) -> bool {
    return !(h <= 23 && m <= 59);
}