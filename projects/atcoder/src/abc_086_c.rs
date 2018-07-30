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
    let mut t: Vec<i32> = vec![0; n + 1];
    let mut x: Vec<i32> = vec![0; n + 1];
    let mut y: Vec<i32> = vec![0; n + 1];

    t[0] = 0;
    x[0] = 0;
    y[0] = 0;
    for i in 1..n+1 {
        t[i] = read();
        x[i] = read();
        y[i] = read();
    }

    let mut succeeded = true;

    for i in 0..n {
        let diff = (t[i+1] - t[i]) - (x[i+1] - x[i]).abs() - (y[i+1] - y[i]).abs();
        if diff < 0 || diff % 2 != 0 {
            succeeded = false;
            break;
        }
    }

    println!("{}", if succeeded { "Yes" } else { "No" });

    // 模範解答: https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-10-%E5%95%8F-abc-086-c---traveling-300-%E7%82%B9
    //let n = read();
    //let mut v: Vec<(i32, i32, i32)> = (0..n).map(|_| (read(), read(), read())).collect();
    //v.insert(0, (0, 0, 0));
    //let yes = v[..].windows(2).all(|w| {
    //    let (t, x, y) = w[0];
    //    let (nt, nx, ny) = w[1];
    //    let time = nt - t;
    //    let dist = (nx - x).abs() + (ny - y).abs();
    //    dist <= time && time % 2 == dist % 2
    //});
    //println!("{}", if yes { "Yes" } else { "No" });
}
