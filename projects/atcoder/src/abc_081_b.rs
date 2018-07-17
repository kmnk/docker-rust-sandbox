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
    let n = read();
    let mut a:Vec<u32> = vec![0; n];

    for i in 0..n {
        a[i] = read();
    }

    let mut cnt = 0;
    'count: loop {
        for i in 0..n {
            if a[i] % 2 != 0 {
                break 'count;
            } else {
                a[i] = a[i] / 2;
            }
        }
        cnt += 1;
    }

    println!("{}", cnt);

    // 回答例: https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-3-%E5%95%8F-abc-081-b---shift-only-200-%E7%82%B9
    //let n = read();
    //println!(
    //    "{}",
    //    (0..n)
    //        .map(|_| read::<u32>().trailing_zeros())
    //        .min()
    //        .unwrap()
    //);

}
