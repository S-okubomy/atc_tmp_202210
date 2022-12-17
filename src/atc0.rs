use itertools::Itertools;
use proconio::{ input };

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let c_vec: Vec<char> = s.chars().collect();
    let mut dq_cnt = 0;
    let mut rep_c_vec: Vec<char> = Vec::new();
    for c in c_vec {
        if c == '"' {
            dq_cnt += 1;
        }
        let mut tmp_c = c;
        if (dq_cnt % 2 == 0) && c == ','  {
            tmp_c = '.'
        }
        rep_c_vec.push(tmp_c);
    }

    println!("{}", rep_c_vec.iter().collect::<String>());
}