use std::collections::HashSet;
fn main() {
    let mut reader = StdinReader::new(std::io::stdin().lock());
    reader.read_space();
    let k: usize = reader.read_line().parse().unwrap();
    let a: HashSet<usize> = reader
        .read_line()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let ans = solve(k, a).to_string();
    write(&ans)
}
fn solve(k: usize, a: HashSet<usize>) -> usize {
    for n in 0..k {
        if a.get(&n).is_none() {
            return n;
        }
    }
    k
}
struct StdinReader<R: std::io::BufRead> {
    reader: R,
    buf: Vec<u8>,
}
impl<R: std::io::BufRead> StdinReader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Vec::new(),
        }
    }
    fn read_space(&mut self) -> String {
        self.read_until(b' ').to_string()
    }
    fn read_line(&mut self) -> String {
        self.read_until(b'\n').to_string()
    }
    fn read_until(&mut self, delim: u8) -> &str {
        loop {
            self.buf.clear();
            let len = self
                .reader
                .read_until(delim, &mut self.buf)
                .expect("failed to reading bytes");
            match len {
                0 => panic!("early eof"),
                1 if self.buf[0] == delim => (),
                _ => {
                    if self.buf[len - 1] == delim {
                        self.buf.truncate(len - 1);
                    }
                    break;
                }
            }
        }
        std::str::from_utf8(&self.buf).expect("invalid utf-8 string")
    }
}
fn write(s: &str) {
    use std::io::{stdout, BufWriter, Write};
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    writeln!(out, "{}", s).unwrap();
}