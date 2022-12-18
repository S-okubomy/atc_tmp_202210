use std::collections::HashSet;

use itertools::Itertools;
use proconio:: { input };
use std::cmp::{ max };

fn main() {
    input! {
        mut k: usize,
    }

    let mut res = 0;
    for (p, e) in get_prime_fact(k) {
        let mut f = 0;
        for n in p..=(10e+12 as usize) {
            f += how_many(n, p);
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





// use std::collections::HashSet;

// use itertools::Itertools;
// use proconio:: { input };
// use std::cmp::{ max };

// fn main() {
//     input! {
//         mut k: usize,
//     }
//     let mut fact = 1;
//     let lim: usize = (k as f64).sqrt() as usize;
//     let mut fact_vec: Vec<(usize, usize)> = Vec::new();
//     let mut ans = 1;
//     let mut n = 0;
//     for p in 2..=lim {
//         if k % p == 0 {
//             let mut a = 0;
//             while k % p == 0 {
//                 a += 1;
//                 k /= a;
//             }
//             n = 0;
//             while a > 0 {
//                 n += p;
//                 let mut x = n;

//                 while x % p == 0 {
//                     x /= p;
//                     a -= 1;
//                 }
//             }
//         }
//         ans = max(ans, n);
//     }
//     ans = max(ans, k);
//     println!("{:?}", ans);
// }

