use std::collections::BTreeSet;
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
fn main() {
    let mut reader = StdinReader::new(std::io::stdin().lock());
    let _n = reader.read_space();
    let q: usize = reader.read_line().parse().unwrap();
    let event: Vec<Vec<usize>> = (0..q)
        .map(|_x| {
            reader
                .read_line()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let ans = solve(event)
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    write(&ans);
}
fn solve(event: Vec<Vec<usize>>) -> Vec<usize> {
    let mut called = BTreeSet::new();
    let mut last = 0;
    let mut ret = Vec::new();
    for e in event {
        match e[0] {
            1 => {
                last += 1;
                called.insert(last);
            }
            2 => {
                called.remove(&e[1]);
            }
            3 => {
                ret.push(*called.first().unwrap());
            }
            _ => unreachable!(),
        }
    }
    ret
}