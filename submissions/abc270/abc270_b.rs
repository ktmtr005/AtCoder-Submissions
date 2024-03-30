use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x: i32,
        y: i32,
        z: i32,
    }
    let ans = solve(x, y, z).unwrap_or(-1);
    println!("{ans}");
}
fn solve(x: i32, y: i32, z: i32) -> Option<i32> {
    let dist_with_hammer = if (y.abs() < z.abs()) && (y.signum() == z.signum()) {
        None
    } else {
        Some(z.abs() + (x - z).abs())
    };
    let dist_without_hammer = if (y.abs() < x.abs()) && (y.signum() == x.signum()) {
        None
    } else {
        Some(x.abs())
    };
    match (dist_with_hammer, dist_without_hammer) {
        (_, Some(n)) => Some(n),
        (Some(n), None) => Some(n),
        (None, None) => None,
    }
}