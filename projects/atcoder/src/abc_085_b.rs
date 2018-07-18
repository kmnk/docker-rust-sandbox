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
    use std::collections::HashSet;

    let n: usize = read();
    let a: HashSet<u32> = (0..n).map(|_| read()).into_iter().collect();
    println!("{}", a.len());

    // 模範解答: https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-7-%E5%95%8F-abc-085-b---kagami-mochi-200-%E7%82%B9
    //use std::collections::BTreeSet;
    //let n = read();
    //let a: BTreeSet<u32> = (0..n).map(|_| read()).collect();
    //println!("{}", a.len());
}
