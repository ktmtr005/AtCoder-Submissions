use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        q: usize,
    }
    for _i in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                k: Usize1,
                x: u32,
            }
            query_1(k, x, &mut a);
        } else {
            input! {
                k: Usize1,
            }
            println!("{}", query_2(k, &a));
        }
    }
}
fn query_1(k: usize, x: u32, a: &mut [u32]) {
    a[k] = x;
}
fn query_2(k: usize, a: &[u32]) -> u32 {
    a[k]
}