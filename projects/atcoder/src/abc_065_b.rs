use std::io::*;
use std::str::FromStr;

fn main() {
    let n: usize = read();
    let a: Vec<usize> = (0..n).map(|_| read::<usize>() - 1).collect();

    let mut cnt = 0;
    let mut b = 0;
    loop {
        cnt = cnt + 1;
        if a[b] == 1 {
            println!("{}", cnt);
            break;
        }
        if cnt >= n {
            println!("-1");
            break;
        }
        b = a[b];
    }
}

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
