use itertools::Itertools;
use proconio:: { input };
use std::collections::{ VecDeque, HashMap, HashSet };
use std::fmt::format;

fn main() {
    input! {
        n: usize, t: usize,
        a_vec: [usize; n],
    }
    let sum_music_sec: usize = a_vec.iter().sum();
    if  t > sum_music_sec {
        let remain_min: usize = t % sum_music_sec;
        let (cnt, p_time) = get_cnt_and_time(a_vec, remain_min);
        println!("{} {}", cnt, p_time);
    } else {
        let (cnt, p_time) = get_cnt_and_time(a_vec, t);
        println!("{} {}", cnt, p_time);
    }
}

fn get_cnt_and_time(a_vec: Vec<usize>, time: usize) -> (usize, usize) {
    let mut cnt = 0;
    let mut sum_min = 0;
    let mut p_time = 0;
    for a in a_vec {
        if sum_min >= time {
            break;
        }
        p_time = time - sum_min;
        sum_min += a;
        cnt += 1;
    }
    (cnt, p_time)
}