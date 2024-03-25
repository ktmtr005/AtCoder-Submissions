use proconio::{fastout, input};
static MOD: i64 = 998244353;
#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
    }
    let ans = solve(a, b, c, d, e, f);
    println!("{ans}");
}
fn solve(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> i64 {
    let [a, b, c, d, e, f] = [a, b, c, d, e, f].map(|x| x % MOD);
    let abc = a * b % MOD * c % MOD;
    let def = d * e % MOD * f % MOD;
    if (abc - def) < 0 {
        (abc - def + MOD) % MOD
    } else {
        (abc - def) % MOD
    }
}