use proconio::{ input };

// cargo run --bin atc1
fn main() {
    input! {
        n: usize, s: usize,
        mut a_vac: [usize; n],
    }

    a_vac.insert(0, 0);
    let mut dp: Vec<Vec<bool>> = vec![vec![false; s+1]; n+1];

    dp[0][0] = true;
    for i in 1..=n {
        let card_no = a_vac[i];
        for j in 0..=s {
            if j >= card_no {
                dp[i][j] = dp[i-1][j] | dp[i-1][j-card_no];
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    println!("{}", if dp[n][s] { "Yes" } else { "No" });
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
}