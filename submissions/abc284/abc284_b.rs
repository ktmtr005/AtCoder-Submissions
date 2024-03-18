fn main() {
    let mut reader = StdinReader::new(std::io::stdin().lock());
    let t: usize = reader.read_line().parse().unwrap();
    let test: Vec<Vec<u32>> = (0..t)
        .map(|_x| {
            reader.read_line();
            reader
                .read_line()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let ans = solve(test)
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    write(&ans);
}