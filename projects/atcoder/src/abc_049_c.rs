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

fn main() {

    // 模範解答のコードリード
    // 各文字列を逆にした char の配列として扱い、更にその配列を patterns とする
    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    // 読み込んだ文字列を逆にして char の配列とする
    let s: Vec<char> = read::<String>().chars().rev().collect();

    // char の配列のスライスを s に入れ直す
    let mut s = &s[..];
    let mut succeeded = true;
    while s.len() > 0 {
        // マッチするパターンを s から探す
        let matched = patterns.iter().find(|&p| s.starts_with(p));

        // マッチした物を Some(p) に入れて None じゃなければ
        if let Some(p) = matched {
            // その長さ文をスライスから消して入れ直す
            s = &s[p.len()..];
        } else {
            // None なら失敗
            succeeded = false;
            break;
        }
    }
    println!("{}", if succeeded { "YES" } else { "NO" });

    // 模範解答: https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-8-%E5%95%8F-abc-085-c---otoshidama-300-%E7%82%B9
    //fn main() {
    //    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
    //        .iter()
    //        .map(|s| s.chars().rev().collect())
    //        .collect();
    //    let s: Vec<char> = read::<String>().chars().rev().collect();
    //    let mut s = &s[..];
    //    let mut succeeded = true;
    //    while s.len() > 0 {
    //        let matched = patterns.iter().find(|&p| s.starts_with(p));
    //        if let Some(p) = matched {
    //            s = &s[p.len()..];
    //        } else {
    //            succeeded = false;
    //            break;
    //        }
    //    }
    //    println!("{}", if succeeded { "YES" } else { "NO" });
    //}
}
