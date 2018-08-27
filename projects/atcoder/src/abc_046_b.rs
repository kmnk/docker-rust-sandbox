use std::io::*;
use std::str::FromStr;

fn main() {
    let n:u32 = read();
    let k:u32 = read();

    println!("{}", k * (k - 1).pow(n - 1));
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
