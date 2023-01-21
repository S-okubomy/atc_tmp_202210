use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize, p: usize, q: usize, r:usize, s: usize,
        mut a_vec: [usize; n],
    }
    // let mut s1_vec: Vec<usize> = a_vec[r-1..s].to_vec();
    // let mut s2_vec: Vec<usize> = a_vec[p-1..q].to_vec();
    // let mut mid_vec: Vec<usize> = a_vec[q..r-1].to_vec();
    // let mut last_vec: Vec<usize> = a_vec[s..n].to_vec();
    // let mut ans: Vec<usize> = Vec::new();

    // ans.append(&mut s1_vec);
    // ans.append(&mut mid_vec);
    // ans.append(&mut s2_vec);
    // ans.append(&mut last_vec);
    // println!("{}", ans.iter().join(" "));

    let len = q - p + 1;
    for i in 0..len {
        a_vec.swap(p-1+i, r-1+i);
    }
    println!("{}", a_vec.iter().join(" "));
}
