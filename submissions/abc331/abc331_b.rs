// abc331/b
use proconio::input;
fn main() {
    input! {
        n: i32,
        s: i32,
        m: i32,
        l: i32,
    }
    let ans = solve(n, s, m, l);
    println!("{}", ans);
}
fn solve(n: i32, s: i32, m: i32, l: i32) -> i32 {
    let mut price_min = std::i32::MAX;
    for i in 0..20 {
        for j in 0..20 {
            for k in 0..20 {
                if 6 * i + 8 * j + 12 * k >= n {
                    let price = i * s + j * m + k * l;
                    if price < price_min {
                        price_min = price;
                    }
                }
            }
        }
    }
    price_min
}