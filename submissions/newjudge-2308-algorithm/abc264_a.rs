fn main() {
    let s = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    proconio::input! {
        L: usize,
        R: usize,
    }
    for count in L-1..R {
        print!("{}", s[count]);
    }
    print!("\n");
}