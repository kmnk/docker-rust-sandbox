use std::io::*;
use std::str::FromStr;

const FILL: char = '#';
const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, -1, 0, 1];

fn main() {
    let h: usize = read();
    let w: usize = read();

    let s: Vec<Vec<char>> = (0..h)
        .map(|_| read::<String>().chars().collect())
        .collect();

    let mut success = true;
    'fullscan: for i in 0..h {
        'nextcell: for j in 0..w {
            if s[i][j] == FILL {
                for k in 0..4 {
                    let x = i as i32 + DX[k];
                    let y = j as i32 + DY[k];

                    if x < 0 || x >= h as i32 { continue 'nextcell; }
                    if y < 0 || y >= w as i32 { continue 'nextcell; }
                    if s[x as usize][y as usize] == FILL { continue 'nextcell; }
                }

                success = false;

                break 'fullscan;
            }
        }
    }

    println!("{}", if success { "Yes" } else { "No" });
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
