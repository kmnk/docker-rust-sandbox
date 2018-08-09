use std::io::*;
use std::str::FromStr;

const EMPTY: char = '.';
const MINE: char = '#';
const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

fn main() {
    let h: usize = read();
    let w: usize = read();

    let mut s: Vec<Vec<char>> = (0..h)
        .map(|_| read::<String>().chars().collect())
        .collect();

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == EMPTY {
                let mut cnt: u32 = 0;

                for k in 0..8 {
                    let x = i as i32 + DX[k];
                    let y = j as i32 + DY[k];

                    if x < 0 || x >= h as i32 { continue; }
                    if y < 0 || y >= w as i32 { continue; }
                    if s[x as usize][y as usize] == MINE {
                        cnt += 1;
                    }
                }

                s[i][j] = std::char::from_digit(cnt as u32, 10).unwrap();
            }
        }
    }

    for i in 0..h {
        let mut string = String::new();
        string.extend(s[i].iter());
        println!("{}", string);
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
