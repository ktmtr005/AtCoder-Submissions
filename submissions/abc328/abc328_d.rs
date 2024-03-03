use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut stack = String::new();
    for i in s {
        stack.push(i);
        if stack.ends_with("ABC") {
            for _i in 0..3 {
                stack.pop();
            }
        }
    }
    if stack.is_empty() == false {
        println!("{}", stack);
    }
}