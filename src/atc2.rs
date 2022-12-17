use std::collections::HashSet;

use itertools::Itertools;
use proconio:: { input };

fn main() {
    input! {
        s_vec: [String; 9],
    }

    let map: Vec<Vec<char>> = s_vec.iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    let mut st_set: HashSet<Vec<(isize, isize)>> = HashSet::new();
    for y1 in 0..9 {
        for x1 in 0..9 {
            for dy in -8..9 {
                for dx in -8..9 {
                    if dy == 0 && dx == 0 { continue; }
                    let (x2, y2) = (x1 + dx, y1 + dy);
                    let (x3, y3) = (x2 - dy, y2 + dx);
                    let (x4, y4) = (x3 - dx, y3 - dy);
                    if is_valid(x1, y1, &map) && is_valid(x2, y2, &map) && is_valid(x3, y3, &map) && is_valid(x4, y4, &map) {
                        // let mut sq_set: HashSet<(isize, isize)> = HashSet::new();
                        // sq_set.insert((x1, y1));
                        // sq_set.insert((x2, y2));
                        // sq_set.insert((x3, y3));
                        // sq_set.insert((x4, y4));
                        let mut sq_vec = vec![(x1, y1), (x2, y2), (x3, y3), (x4, y4)];
                        sq_vec.sort();
                        // println!("{:?}", sq_vec);
                        st_set.insert(sq_vec);
                    }
                } 
            }

            // let mut dy: isize = -8;
            // while dy <= 8 {
            //     let mut dx = -8;
            //     while dx <= 8 {
            //         let (x2, y2) = (x1 + dx, y1 + dy);
            //         let (x3, y3) = (x2 - dy, y2 + dx);
            //         let (x4, y4) = (x3 - dx, y3 - dy);
            //         if is_valid(x1, y1, &map) && is_valid(x2, y2, &map) && is_valid(x3, y3, &map) && is_valid(x4, y4, &map) {
            //             let mut sq_set: HashSet<(isize, isize)> = HashSet::new();
            //             sq_set.insert((x1, y1));
            //             sq_set.insert((x2, y2));
            //             sq_set.insert((x3, y3));
            //             sq_set.insert((x4, y4));
            //             st_set.insert(sq_set);
            //         }



            //         dx += 1;
            //     }
            //     dy += 1;
            // }
        }
    }
    println!("{}", st_set.len());
}

fn is_valid(x: isize, y: isize, map: &Vec<Vec<char>>) -> bool {
    (0 <= x && x <= 8) && (0 <= y && y <= 8) && map[x as usize][y as usize] == '#'
}

fn is_square1(posi_x: usize, posi_y: usize, map: Vec<Vec<char>>) -> bool {
    for y in posi_y..9 {
        for x in posi_x..9 {
            let (nx, ny) = (posi_x + x, posi_y + x);
            if nx > 9 || ny > 9 {
                return  false;
            }
            if map[y][x] == '#' && map[ny][x] == '#' && map[ny][nx] == '#' {
                return true;
            }
        }
    }
    false
}

fn is_square2(posi_x: usize, posi_y: usize, map: Vec<Vec<char>>) -> bool {
    for y in posi_y..9 {
        for x in posi_x..9 {
            let (nx, ny) = (posi_x - x, posi_y + x);
            if nx > 9 || ny > 9 {
                return  false;
            }
            if map[y][x] == '#' && map[ny][x] == '#' && map[ny][nx] == '#' {
                return true;
            }
        }
    }
    false
}





// fn main() {
//     input! {
//         a: usize, b:usize, c: usize, d: usize, e: usize, f:usize,
//     }
//     const MOD: usize = 998244353;
//     let ans = (
//           (((a % MOD) * (b % MOD)) % MOD * ((c % MOD))) % MOD + MOD
//         - (((d % MOD) * (e % MOD)) % MOD * ((f % MOD))) % MOD
//     ) % MOD;

//     // println!("{}", ((a % MOD) * (b % MOD)) % MOD  * (c % MOD));

//     println!("{}", ans);
// }