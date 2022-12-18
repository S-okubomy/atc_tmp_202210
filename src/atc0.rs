use itertools::Itertools;
use proconio::{ input };

fn main() {
    input! {
        n: usize, m: usize,
        nv_vec: [(usize, usize); m],
    }

    let mut nb_vec: Vec<Vec<usize>> = vec![vec![]; n+1];
    for nv in nv_vec {
        let (n, v) = nv;
        nb_vec[n].push(v);
        nb_vec[v].push(n);
    }


// TODO 修正
// https://atcoder.jp/contests/abc282/editorial/5397

    let mut is_bipartite: bool = true;
    let mut color: Vec<isize> = vec![-1; n+1];
    let mut cnt = 0;
    for i in 0..n {
        if color[i] != -1 {
            continue;
        }
        // if !dfs(&nb_vec, i, 0, &mut color) {
        //     is_bipartite = false;
        // }
        if dfs(&nb_vec, i, 0, &mut color) {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

fn dfs(nb_vec: &Vec<Vec<usize>>, v: usize, cur: isize, color: &mut Vec<isize>) -> bool {
    color[v] = cur;
    for next_v in &nb_vec[v] {
        if color[*next_v] != -1 {
            if color[*next_v] == cur {
                return false;
            }
            continue;
        }

        if !dfs(nb_vec, *next_v, 1 - cur, color) {
            return false;
        }
    }
    true
}