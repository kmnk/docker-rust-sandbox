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
    let a: Vec<Vec<usize>> = (0..2)
        .map(|_| (0..n).map(|_| read::<usize>()).collect())
        .collect();

    let mut max = 0;
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..n {
            if j < i      { cnt += a[0][j]; }
            else if j > i { cnt += a[1][j]; }
            else          { cnt += a[0][j] + a[1][j]; }
        }
        if cnt > max { max = cnt; }
    }

    println!("{}", max);
}
