use proconio::{ input };

// cargo run --bin atc1
fn main() {
    input! {
        n: usize,
    }
    println!("{}", fact(n));
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
}