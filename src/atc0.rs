use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = 0;
    let mut is_renzoku = false;
    for i in 0..s.len() - 1 {
        if s[i] == '0' && s[i+1] == '0' && !is_renzoku {
            cnt += 1;
            is_renzoku = true;
        } else {
            is_renzoku = false;
        }
    }
    println!("{}", s.len() - cnt);
}
