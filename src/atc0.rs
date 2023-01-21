use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize, a: usize, b: usize,
        s_vec: Chars,
    }

    let mut sum = 0;
    if s_vec[0] == s_vec[1] {
        sum += a;
        if n % 2 != 0 {
            sum += (n - 3) / 2 * b;  
        } else {
            sum += (n - 2) / 2 * b;
        }
    } else {
        if n % 2 != 0 {
            sum += (n - 1) / 2 * b;  
        } else {
            sum += n / 2 * b;
        }
    }
    println!("{}", sum);
}
