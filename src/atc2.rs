use itertools::Itertools;
use proconio:: { input };
use std::{collections::{ VecDeque }, fmt::format};

fn main() {
    input! {
        mut h: usize, mut m: usize,
    }
    for _ in 0..24 {
        for _ in 0..60 {
            let (hs, ms) = (format!("{:0>2}", h), format!("{:0>2}", m));
            let (tl, tr, dl, dr) = (&hs[0..=0], &ms[0..=0], &hs[1..=1], &ms[1..=1]);
            let h_cnv: usize = format!("{:0>2}", tl.to_string() + tr).parse().ok().unwrap();
            let m_cnv: usize = format!("{:0>2}", dl.to_string() + dr).parse().ok().unwrap();
            if h_cnv <= 23 && m_cnv <=59 {
                println!("{} {}", h, m);
                return;
            };
            m += 1;
            if m >= 60 {
                m = 0;
                h += 1;
            }
            if h >= 24 {
                h = 0;
            }
        }
    }
}