use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        b: i64,
    }
    let mut ans = false;
    let mut a: i64 = 1;
    while a <= 15 {
        let mut aa = a;
        for _i in 2..=a {
            aa *= a;
        }
        if aa == b {
            ans = true;
            break;
        }
        a += 1;
    }
    if ans == true {
        println!("{}", a);
    }
    else {
        println!("-1");
    }
}