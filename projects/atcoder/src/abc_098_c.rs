use std::io::*;
use std::str::FromStr;
use std::cmp;

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

const E: char = 'E';
const W: char = 'W';

fn main() {
    let n: usize = read();
    let s: Vec<char> = read::<String>().chars().collect();


    let mut right = 0;
    for i in 0..n {
        if s[i] == E { right += 1; }
    }

    let mut min = n;

    let mut left = 0;
    for i in 0..n {
        if s[i] == E { right -= 1; }
        if i > 0 && s[i-1] == W { left += 1; }

        min = cmp::min(min, left + right);
    }

    println!("{}", min);
}
