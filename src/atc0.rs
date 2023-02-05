use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize, m: usize,
        ab_vec: [(usize, usize); m],
    }

    let mut nb_vec: Vec<Vec<usize>> = vec![vec![]; n+1];
    for (a, b) in ab_vec {
        nb_vec[a].push(b);
        nb_vec[b].push(a);
    }

    let mut is_passed: Vec<bool> = vec![false; n+1];

    println!("{:?}", nb_vec);

    // println!("{}", dfs(1, 0, &mut is_passed, &nb_vec));

    let mut visited: Vec<bool> = vec![false; n+1]; // false: 通ってない、true: 通った
    dfs(&mut visited, &nb_vec, 1);

    let mut cnt = 0;

    for v in visited.iter().skip(1) {
        if *v {
            cnt += 1;
        } 
    }
    println!("{}", cnt);
}

fn dfs(visited: &mut Vec<bool>, neighber_nodes: &Vec<Vec<usize>>, pos: usize) -> () {
    visited[pos] = true;
    for neighber_n in &neighber_nodes[pos] {
        if !visited[*neighber_n] { // 頂点通っていなければ、通る
            dfs(visited, neighber_nodes, *neighber_n)
        }
    }
}


// fn dfs(cur: usize, bef: usize, is_passed: &mut Vec<bool>, nb_vec: &Vec<Vec<usize>>) -> usize {
//     let mut cnt = 0;
//     is_passed[cur] = true;
//     for i in 1..nb_vec.len() {
//         if nb_vec[cur][i] == bef {
//             continue;
//         }
//         // println!("確認");
//         if is_passed[nb_vec[cur][i]] {
//             println!("pass");

//             cnt += 1;
//         }
//         dfs(cur, bef, is_passed, nb_vec);
//     }

//     cnt
// }
