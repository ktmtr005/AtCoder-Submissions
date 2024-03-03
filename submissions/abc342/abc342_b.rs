use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u32; n],
        q: usize,
        ab: [(u32, u32); q],
    }
    for (a, b) in ab {
        let a_pos = p.iter().position(|&x| x == a).unwrap();
        let b_pos = p.iter().position(|&x| x == b).unwrap();
        println!("{}", p[a_pos.min(b_pos)]);
    }
}