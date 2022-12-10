use itertools::Itertools;
use proconio:: { input };
use std::collections::{ VecDeque, HashMap, HashSet };
use std::fmt::format;


fn main() {
    input! {
        _n: usize, q: usize,
        tab_vec: [(usize, usize, usize); q],
    }

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for tab in tab_vec {
        let (t, a, b) = tab;
        match t {
            1 => {
                map.entry(a).or_default().insert(b);
            },
            2 => {
                map.entry(a).and_modify(|s| { s.remove(&b); });
            },
            3 => {
                if map.contains_key(&a) && map.contains_key(&b) {
                    if map.get(&a).unwrap().contains(&b) && map.get(&b).unwrap().contains(&a) {
                        println!("Yes");
                    } else {
                        println!("No");
                    }
                } else {
                    println!("No");
                }
            },
            _ => (),
        }
    }
}