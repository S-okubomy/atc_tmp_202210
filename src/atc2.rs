use std::collections::HashSet;

use itertools::Itertools;
use proconio:: { input };
use std::cmp::{ max };

fn main() {
    input! {
        mut k: usize,
    }

    let mut res = 0;
    let prim_vec = get_prime_fact(k);
    println!("{:?}" , prim_vec);
    for (p, e) in prim_vec {
        let mut f = 0;
        for n in p..=(10e+12 as usize) {
            println!("{} {}", n, p);
            f += how_many(n, p);
            println!("f: {}", f);
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
    let mut res = 0;
    while n % p == 0 {
        n /= p;
        res += 1;
    }
    res
}



