use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut a_vec: [usize; n],
        q_size: usize,
    }

    let mut ans_vec: Vec<String> = Vec::new();
    for _ in 0..q_size {
        input! { q_type: usize, k: usize };
        // println!("確認: {}", q_type);
        if q_type == 1 {
            input! { x: usize };
            a_vec[k-1] = x;
        } else if q_type == 2 {
            ans_vec.push(a_vec[k-1].to_string());
        }
    }
    println!("{}", ans_vec.join("\n"));

    // for q in q_vec {
    //     let qq = q.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    //     println!("{:?}", qq);
    //     let k = qq[1].parse::<usize>().ok().unwrap();
    //     if qq[0] == "1" {
    //         a_vec[k-1] = qq[2].parse().ok().unwrap();
    //     } else {
    //         println!("{}", a_vec[k-1]);
    //     }
    // }
}
