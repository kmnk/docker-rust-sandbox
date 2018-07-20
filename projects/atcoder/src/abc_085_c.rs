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
    let n: i32 = read();
    let y: i32 = read();

    let mut a: i32 = -1;
    let mut b: i32 = -1;
    let mut c: i32 = -1;

    for i in 0..n+1 {
        for j in 0..n+1-i {
            let k = n - i - j;
            if (10000 * i) + (5000 * j) + (1000 * k) == y {
                a = i;
                b = j;
                c = k;
                break;
            }
        }
    }

    println!("{} {} {}", a, b, c);

    // 模範解答: https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-8-%E5%95%8F-abc-085-c---otoshidama-300-%E7%82%B9
    //let n: i32 = read();
    //let x: i32 = read();
    //let mut ans = None;
    //'outer: for i in 0..n + 1 {
    //    for j in 0..n - i + 1 {
    //        let k = n - i - j;
    //        if i * 10000 + j * 5000 + k * 1000 == x {
    //            ans = Some((i, j, k));
    //            break 'outer;
    //        }
    //    }
    //}
    //let (x, y, z) = ans.unwrap_or((-1, -1, -1));
    //println!("{} {} {}", x, y, z);
}
