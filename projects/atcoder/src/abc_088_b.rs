use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let n: usize = read();
    let mut a: Vec<u32> = vec![0; n];

    for i in 0..n {
        a[i] = read();
    }
    a.sort();
    a.reverse();

    let mut alice = 0;
    let mut bob = 0;

    for i in 0..n {
        if i % 2 == 0 {
            alice += a[i];
        } else {
            bob += a[i];
        }
    }

    println!("{}", alice - bob);

    // 回答例: https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-6-%E5%95%8F-abc-088-b---card-game-for-two-200-%E7%82%B9
    //    let n = read();
    //    let mut a: Vec<u32> = (0..n).map(|_| read()).collect();
    //    a.sort_by(|x, y| y.cmp(x));
    //    let mut alice = 0;
    //    let mut bob = 0;
    //    for (i, &x) in a.iter().enumerate() {
    //        if i % 2 == 0 {
    //            alice += x;
    //        } else {
    //            bob += x;
    //        }
    //    }
    //    println!("{}", alice - bob);
}
