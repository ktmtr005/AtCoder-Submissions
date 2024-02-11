use proconio::input;
fn main() {
    input! {
        n: i32,
        a:[i64;n],
    }
    let mut bit_or: i64 = 0;
    for i in a {
        assert!(i >= 1 && i <= 1000000000);
        bit_or = bit_or | i;
    }
    let mut cnt: i32 = 0;
    while (bit_or & 1) == 0 {
        bit_or = bit_or >> 1;
        cnt += 1;
    }
    println!("{}", cnt);
}