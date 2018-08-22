use std::io::*;
use std::str::FromStr;

fn main() {
    let n:u64 = read();

    let mut p = 1;
    for i in 0..n {
        p = ((i + 1) * p)  % (10u64.pow(9) + 7);
    }

    println!("{}", p);
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
