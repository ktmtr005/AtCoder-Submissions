fn main() {
    let (_n, s, t) = input();
    match solve(&s, &t) {
        true => println!("Yes"),
        false => println!("No"),
    }
}
fn input() -> (usize, String, String) {
    let nst: Vec<String> = (0..3).map(|_| {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line.");
        line.chars().collect()
    }).collect::<Vec<String>>();
    let mut iter = nst.iter();
    (
        iter.next().unwrap().trim().parse().unwrap(),
        iter.next().unwrap().trim().chars().collect(),
        iter.next().unwrap().trim().chars().collect()
    )
}
fn solve(s: &String, t: &String) -> bool {
    let s_sim = s.replace("1", "l").replace("0", "o");
    let t_sim = t.replace("1", "l").replace("0", "o");
    s_sim == t_sim
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_works() {
        input();
    }
    #[test]
    fn solve_works() {
        assert_eq!(solve(&"abc".to_string(), &"arc".to_string()), false)
    }
}