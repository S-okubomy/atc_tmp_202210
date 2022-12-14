use std::collections::{HashSet, HashMap, VecDeque};

use itertools::Itertools;
use proconio:: { input, marker::Chars };
use std::cmp::{ max, min };


fn main() {
    input! {
        mut s_vec: Chars,
    }
    s_vec.insert(0, '$');

    if s_vec[1] == '1' {
        println!("No");
        return;
    }

    let mut column: Vec<bool> = vec![false; 7];
    column[0] = s_vec[7] == '1';
    column[1] = s_vec[4] == '1';
    column[2] = s_vec[2] == '1' || s_vec[8] == '1';
    column[3] = s_vec[1] == '1' || s_vec[5] == '1';
    column[4] = s_vec[3] == '1' || s_vec[9] == '1';
    column[5] = s_vec[6] == '1';
    column[6] = s_vec[10] == '1';

    for i in 0..7 {
        for j in 0..i {
            if column[i] && column[j] {
                for k in j+1..i {
                    if !column[k] {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}





fn dfs_sample1(ab_vec: Vec<(usize, usize)>) -> usize {
    // https://atcoder.jp/contests/abc277/tasks/abc277_c
    let mut nb_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (a, b) in ab_vec {
        nb_map.entry(a).or_default().insert(b);
        nb_map.entry(b).or_default().insert(a);
    }

    println!("{:?}", nb_map);

    let mut visited: HashSet<usize> = HashSet::new();
    dfs(1, &mut visited, &nb_map);

    *visited.iter().max().unwrap()
}

fn dfs(pos: usize, visited: &mut HashSet<usize>, nb_map: &HashMap<usize, HashSet<usize>>) {
    if visited.contains(&pos) { return; }
    
    visited.insert(pos);

    if !nb_map.contains_key(&pos) { return; }
    for nb in nb_map.get(&pos).unwrap() {
        if !visited.contains(nb) {
            dfs(*nb, visited, &nb_map)
        }
    }
}


fn bfs_sample1(ab_vec: Vec<(usize, usize)>) -> usize {
    // https://atcoder.jp/contests/abc277/tasks/abc277_c
    let mut nb_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (a, b) in ab_vec {
        nb_map.entry(a).or_insert(HashSet::new()).insert(b);
        nb_map.entry(b).or_default().insert(a);
    }

    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut fl_set: HashSet<usize> = HashSet::new();
    deque.push_back(1);
    fl_set.insert(1);
    while deque.len() > 0 {
        let f_num = deque.pop_front().unwrap();
        if !nb_map.contains_key(&f_num) { continue; }
        for next_fl in nb_map.get(&f_num).unwrap() {
            if !fl_set.contains(next_fl) {
                fl_set.insert(*next_fl);
                deque.push_back(*next_fl);
            }
        }
    }
    *fl_set.iter().max().unwrap()
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


// n?????????e?????????????????????
// https://drken1215.hatenablog.com/entry/2022/12/05/160400
fn how_many(mut n: usize, e: usize) -> usize {
    // ???????????????
    // 1!=1
    // 2!=2
    // 3!=3^1??2 (?????????)
    // 4!=3^1??8
    // 5!=3^1??40
    // 6!=3^2??80 (?????????)
    // 7!=3^2??560
    // 8!=3^2??4480
    // 9!=3^4??4480 (?????????)????????????6!?????????+2?????????

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

    #[test]
    fn test3() {
        let ab_vec: Vec<(usize, usize)> = vec![(1,4), (4,3), (4, 10), (8,3)];
        assert_eq!(bfs_sample1(ab_vec), 10);
        let ab_vec: Vec<(usize, usize)> = vec![(1,3), (1,5), (1,12), (3,5), (3,12), (5,12)];
        assert_eq!(bfs_sample1(ab_vec), 12);
    }

    #[test]
    fn test4() {
        let ab_vec: Vec<(usize, usize)> = vec![(1,4), (4,3), (4, 10), (8,3)];
        assert_eq!(dfs_sample1(ab_vec), 10);
        let ab_vec: Vec<(usize, usize)> = vec![(1,3), (1,5), (1,12), (3,5), (3,12), (5,12)];
        assert_eq!(dfs_sample1(ab_vec), 12);
    }
}