use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    
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
