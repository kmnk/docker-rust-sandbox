use std::io::*;
use std::str::FromStr;

fn main() {
    let a:i32 = read();
    let b:i32 = read();
    let c:i32 = read();

    let mut s = false;
    for i in 1..b+1 {
        if (a * i) % b == c {
            s = true;
            break;
        }
    }

    println!("{}", if s { "YES" } else { "NO" });
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
