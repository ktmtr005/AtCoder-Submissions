use proconio::{input, fastout, marker::Chars};
#[fastout]
fn main() {
  input! {
    mut s: Chars,
  }
  for i in 0..s.len() {
    if s[i] == '0' {
      s[i] = '1';
    }
    else {
      s[i] = '0';
    }
  }
  println!("{}", s.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(""));
}