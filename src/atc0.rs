use proconio::{ input };

fn main() {
    input! {
        n: usize, m: usize,
        s: [String; n],
    }
    let s_vec: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect()).collect();

    let mut count = 0;
    for i in 0..n {
        for j in (i+1)..n {
            if is_ok(&s_vec[i], &s_vec[j], m) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn is_ok(p1: &Vec<char>, p2: &Vec<char>, m: usize) -> bool {
    for i in 0..m {
        if p1[i] == 'x' && p2[i] == 'x' {
            return false;
        }
    }
    return true;
}