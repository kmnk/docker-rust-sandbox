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
    let n: u32 = read();
    let a: u32 = read();
    let b: u32 = read();

    let mut ans = 0;

    for v in 1..n+1 {
        let c_10000 = v / 10000;
        let c_1000  = (v - (10000 * c_10000)) / 1000;
        let c_100   = (v - (10000 * c_10000) - (1000 * c_1000)) / 100;
        let c_10    = (v - (10000 * c_10000) - (1000 * c_1000) - (100 * c_100)) / 10;
        let c_1     = (v - (10000 * c_10000) - (1000 * c_1000) - (100 * c_100) - (10 * c_10)) / 1;
        let c = c_10000 + c_1000 + c_100 + c_10 + c_1;

        if a <= c && c <= b {
            ans += v;
        }
    }

    println!("{}", ans);

    // 回答例: https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-5-%E5%95%8F-abc-083-b---some-sums-200-%E7%82%B9
    //let ans = (1..n + 1)
    //    .filter(|x| {
    //        let sum = x.to_string()
    //            .chars()
    //            .map(|c| (c as u8 - b'0') as u32) // u8 でバイト数値にして b'0' を引けば対応する数値になる
    //            .sum::<u32>();
    //        a <= sum && sum <= b
    //    })
    //    .sum::<u32>();
    //println!("{}", ans);

}
