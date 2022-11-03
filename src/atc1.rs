use proconio::{ input };

// cargo run --bin atc1
fn main() {
    input! {
        n: usize, s: usize,
    }
    let ans: Vec<usize> = (1..=n).map(|i| {
            let mut a_vec: Vec<usize> = Vec::new();
            for j in 1..=n { a_vec.push(i+j) };
            return a_vec; 
        })
        .flat_map(|v| v)
        .filter(|x| x <= &s)
        .collect();
    
    println!("{:?}", ans);

    // let cnt: usize = (1..=n).map(|i| {
    //         let mut a_vec: Vec<usize> = Vec::new();
    //         for j in 1..=n { a_vec.push(i+j) };
    //         return a_vec; 
    //     })
    //     .flat_map(|v| v)
    //     .filter(|x| x <= &s)
    //     .count();
    // println!("{:?}", cnt);

    let ans2: Vec<usize> = (1..=n).map(|i| (1..=n).map(|j| j+i).collect::<Vec<usize>>())
            .flatten()
            .filter(|x| x <= &s)
            .collect();
    println!("{:?}", ans2);

    let cnt: usize = (1..=n).map(|i| (1..=n).map(|j| j+i).collect::<Vec<usize>>())
                            .flat_map(|v| v)
                            .filter(|x| x <= &s)
                            .count();
    println!("{}", cnt);
}

fn fact(x: usize) -> usize {
    match x {
        0 | 1 => 1,
        _ => x * fact(x-1),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { return a; }
    gcd(b, a%b)
}



#[allow(dead_code)]
fn apple() {
    input! {
        x: usize, y: usize, n: usize,
    }

    println!("{}", if 3 * x < y { n *x } else { (n/3) * y + (n%3 * x) });
}

#[allow(dead_code)]
fn apple2() {
    input! {
        x: usize, y: usize, n: usize,
    }

    let mut min_money = 10000;
    for j in 0..n {
        if n < 3 * j { continue; }
        let m =  x * (n - 3 * j) + y * j;
        if min_money > m {
            min_money = m;
        }
    }
    println!("{}", min_money);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  test1() {
        assert_eq!(gcd(12, 18), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(lcm(3, 4), 12);
    }

    #[test]
    fn test3() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(3), 6);
        assert_eq!(fact(5), 120);
    }

    #[test]
    fn test4() {
        // https://atcoder.jp/contests/math-and-algorithm/submissions/36177376
        let (n, s) = (3, 4);
        let ans1: Vec<usize> = (1..=n).map(|i| (1..=n).map(|j| j+i).collect::<Vec<usize>>())
                                    .flatten().filter(|x| x <= &s).collect();
        println!("{:?}", ans1);
        assert_eq!(ans1, vec![2, 3, 4, 3, 4, 4]);

        let ans2: Vec<usize> = (1..=n).map(|i| (1..=n).map(|j| j+i).collect::<Vec<usize>>())
                                    .flat_map(|v| v).filter(|x| x <= &s).collect();
        assert_eq!(ans2, vec![2, 3, 4, 3, 4, 4]);
    }
}