use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }
    let mut g = vec![Vec::new(); n];
    for (i, u) in a.iter().enumerate() {
        for (j, &v) in u.iter().enumerate() {
            if v == 1 {
                g[i].push(j+1);
            }
        }
    }
    for ln in g {
        println!("{}", ln.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
    }
}