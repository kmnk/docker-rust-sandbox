use std::io::*;
use std::str::FromStr;

fn main() {
    let a:i64 = read();
    let b:i64 = read();
    let x:i64 = read();

    println!("{}", f(b, x) - f(a - 1, x));
}

fn f(v:i64, x:i64) -> i64 {
    if v < 0 { return 0; }
    else { return v / x + 1; }
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
