use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
        q: usize,
        cd: [(char, char); q]
    }
    let mapping_from = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut mapping_to = mapping_from.clone();
    for (c, d) in cd {
        mapping_to = mapping_to.replace(c, d.to_string().as_str());
    }
    let mut ans = String::new();
    for c in s.chars() {
        let i = mapping_from.find(c).unwrap();
        ans.push(mapping_to.chars().nth(i).unwrap());
    }
    println!("{}", ans);
}