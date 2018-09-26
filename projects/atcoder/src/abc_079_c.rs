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

const P: char = '+';
const M: char = '-';

fn main() {
    let s: Vec<i32> = read::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let op: [[i32; 3]; 8] = [
        [0, 0, 0],
        [0, 0, 1],
        [0, 1, 0],
        [0, 1, 1],
        [1, 0, 0],
        [1, 0, 1],
        [1, 1, 0],
        [1, 1, 1],
    ];

    for i in 0..8 {
        let mut ans = s[0];
        if op[i][0] == 0 { ans += s[1]; } else { ans -= s[1]; }
        if op[i][1] == 0 { ans += s[2]; } else { ans -= s[2]; }
        if op[i][2] == 0 { ans += s[3]; } else { ans -= s[3]; }
        if ans == 7 {
            println!("{}{}{}{}{}{}{}=7",
                     s[0],
                     if op[i][0] == 0 { P } else { M },
                     s[1],
                     if op[i][1] == 0 { P } else { M },
                     s[2],
                     if op[i][2] == 0 { P } else { M },
                     s[3]);
            break;
        }
    }

}
