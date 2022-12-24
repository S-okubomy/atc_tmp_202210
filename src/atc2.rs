use std::collections::{HashSet, HashMap};

use itertools::Itertools;
use proconio:: { input, marker::Chars };
use std::cmp::{ max };

fn main() {    
    input! {
        h: usize, w: usize,
        s_vec: [Chars; h],
        t_vec: [Chars; h],
    }

    let mut s_h_vec: Vec<Vec<char>> = Vec::new();
    let mut t_h_vec: Vec<Vec<char>> = Vec::new();
    for j in 0..w {
        s_h_vec.push(s_vec.iter().map(|c| c[j]).collect::<Vec<char>>());
        t_h_vec.push(t_vec.iter().map(|c| c[j]).collect::<Vec<char>>());

    }
    s_h_vec.sort();
    t_h_vec.sort();

    println!("{}", if s_h_vec == t_h_vec { "Yes" } else { "No" });

    // let mut move_map: HashMap<usize, usize> = HashMap::new();
    // for j_t in 0..w {
    //     let to = move_map.entry(j_t).or_default();
    //     for j_s in 0..w {
    //         if t_vec[0][j_t] == s_vec[0][j_s] {
    //             *to = j_s;
    //             s_vec[0][j_s] = '-'; 
    //         }
    //     } 
    // }
}

fn get_prime_fact(x: usize) -> Vec<(usize, usize)> {
    let mut k = x;
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
    if k != 1 {
        fact_vec.push((k, 1));
    }
    fact_vec
}


// nを指数eで何回割れるか
// https://drken1215.hatenablog.com/entry/2022/12/05/160400
fn how_many(mut n: usize, e: usize) -> usize {
    // 指数に注目
    // 1!=1
    // 2!=2
    // 3!=3^1×2 (更新！)
    // 4!=3^1×8
    // 5!=3^1×40
    // 6!=3^2×80 (更新！)
    // 7!=3^2×560
    // 8!=3^2×4480
    // 9!=3^4×4480 (更新！)　指数が6!に比べ+2してる

    let mut res = 0;
    while n % e == 0 {
        n /= e;
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(how_many(8, 2), 3);
        assert_eq!(how_many(4, 2), 2);
        assert_eq!(how_many(2, 2), 1);
        assert_eq!(how_many(3, 3), 1);
        assert_eq!(how_many(6, 3), 1);
        assert_eq!(how_many(9, 3), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(get_prime_fact(30), vec![(2,1), (3,1), (5,1)]);
        assert_eq!(get_prime_fact(123456789011), vec![(123456789011,1)]);
        assert_eq!(get_prime_fact(280), vec![(2,3),(5,1),(7,1)]);
    }

}