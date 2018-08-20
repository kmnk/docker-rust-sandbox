use std::io::*;
use std::str::FromStr;
use std::cmp;

fn main() {
    let a: [i16; 2] = [read(), read()];
    let b: [i16; 2] = [read(), read()];

    let l = cmp::max(a[0], b[0]);
    let r = cmp::min(a[1], b[1]);

    println!("{}", cmp::max(0, r - l));
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
