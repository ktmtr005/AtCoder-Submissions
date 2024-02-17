use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        st: [(usize, usize); n - 1],
    }
    for (i, &(s, t)) in st.iter().enumerate() {
            a[i + 1] += (a[i] / s) * t;
    }
    println!("{}", *a.iter().last().unwrap());
}