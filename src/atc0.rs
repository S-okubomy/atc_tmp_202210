use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let ans: String = s.replace("na", "nya");
    println!("{}", ans);
}
