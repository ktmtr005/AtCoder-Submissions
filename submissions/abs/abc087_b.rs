use proconio::input;
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }
    let ans = solve(a, b, c, x);
    println!("{}", ans);
}
fn solve(a: i32, b: i32, c: i32,  x: i32) -> i32 {
    let mut ans: i32 = 0;
    for i in 0..=a {
        for j in 0..=b {
            let k = (x - (500 * i + 100 * j)) / 50;
            if k >= 0 && k <= c {
                debug_assert!((x - (500 * i + 100 * j)) % 50 == 0);
                ans += 1;
            }
        }
    }
    ans
}