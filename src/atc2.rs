use std::collections::HashSet;

use itertools::Itertools;
use proconio:: { input };
use std::cmp::{ max };

// https://drken1215.hatenablog.com/entry/2022/12/05/160400
fn main() {
    input! {
        mut k: usize,
    }

    let mut res = 0;
    let prim_vec = get_prime_fact(k);
    // println!("{:?}" , prim_vec);
    for (p, e) in prim_vec {
        let mut f = 0;
        for n in p..=p*e {
            // println!("{} {}", n, p);
            f += how_many(n, p);
            // println!("f: {}", f);
            if f >= e {
                res = max(res, n);
                break;
            }
        }
    }
    println!("{:?}", res);
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

fn how_many(mut n: usize, p: usize) -> usize {
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
    while n % p == 0 {
        n /= p;
        res += 1;
    }
    res
}