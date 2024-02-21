use std::collections::{HashMap, VecDeque};
fn read() -> VecDeque<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().split_whitespace().map(|s| s.to_string()).collect::<VecDeque<String>>()
}
fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();
        line.trim().parse::<usize>().ok().unwrap()
    };
    let mut variables: HashMap<String, i32> = HashMap::new();
    let mut vectors: HashMap<String, Vec<i32>> = HashMap::new();
    for _i in 0..n {
        let line = read();
        calculator(&mut variables, &mut vectors, line);
    }
}
fn calculator(map: &mut HashMap<String, i32>, map_vec: &mut HashMap<String, Vec<i32>>, mut line: VecDeque<String>) {
    let instruction = line.pop_front().unwrap();
    let instruction_type = if instruction == "int" {
        1
    }
    else if instruction == "print_int" {
        2
    }
    else if instruction == "vec" {
        3
    }
    else if instruction == "print_vec" {
        4
    }
    else {
        unreachable!()
    };
    match instruction_type {
        1 => {
            let x = line.pop_front().unwrap();
            line.pop_front();
            let mut calc = 0;
            let mut ope = 1; // 1: 加算, 2: 減算
            loop {
                let v = line.pop_front().unwrap();
                match v.as_str() {
                    "+" => ope = 1,
                    "-" => ope = -1,
                    ";" => break,
                    _ => if let Ok(i) = v.parse::<i32>() {
                        calc += ope * i;
                    }
                    else {
                        calc += ope * map.get(&v).unwrap().clone();
                    },
                }
            }
            match map.get(&x) {
                Some(&_i) => unreachable!(),
                None => map.insert(x, calc),
            };
        },
        2 => {
            let mut calc = 0;
            let mut ope = 1; // 1: 加算, 2: 減算
            loop {
                let v = line.pop_front().unwrap();
                match v.as_str() {
                    "+" => ope = 1,
                    "-" => ope = -1,
                    ";" => break,
                    _ => if let Ok(i) = v.parse::<i32>() {
                        calc += ope * i;
                    }
                    else {
                        calc += ope * map.get(&v).unwrap().clone();
                    },
                }
            }
            println!("{}", calc);
        }
        3 => {
            let a = line.pop_front().unwrap();
            line.pop_front();
            let mut v = Vec::new();
            let mut new_vec = Vec::new();
            let mut is_vec = false;
            let mut ope = 1;
            loop {
                let s = line.pop_front().unwrap();
                match s.as_str() {
                    "[" => is_vec = true,
                    "]" => {
                        is_vec = false;
                        if v.is_empty() {
                            v = new_vec.clone();
                        }
                        else {
                            for i in 0..v.len() {
                                v[i] += ope * new_vec[i];
                            }
                        }
                        new_vec.clear();
                    },
                    "," => continue,
                    ";" => break,
                    "+" => ope = 1,
                    "-" => ope = -1,
                    _ => if let Ok(i) = s.parse::<i32>() {
                        new_vec.push(i);
                    }
                    else {
                        if is_vec == true {
                            new_vec.push(map.get(&s).unwrap().clone());
                        }
                        else {
                            let tmp_vec = map_vec.get(&s).unwrap().clone();
                            if v.is_empty() {
                                v = tmp_vec.clone();
                            }
                            else {
                                for i in 0..v.len() {
                                    v[i] += ope * tmp_vec[i];
                                }
                            }
                        }
                    }
                }
            }
            map_vec.insert(a, v);
        },
        4 => {
            let mut v = Vec::new();
            let mut new_vec = Vec::new();
            let mut is_vec = false;
            let mut ope = 1;
            loop {
                let s = line.pop_front().unwrap();
                match s.as_str() {
                    "[" => is_vec = true,
                    "]" => {
                        is_vec = false;
                        if v.is_empty() {
                            v = new_vec.clone();
                        }
                        else {
                            for i in 0..v.len() {
                                v[i] += ope * new_vec[i];
                            }
                        }
                        new_vec.clear();
                    },
                    "," => continue,
                    ";" => break,
                    "+" => ope = 1,
                    "-" => ope = -1,
                    _ => if let Ok(i) = s.parse::<i32>() {
                        new_vec.push(i);
                    }
                    else {
                        if is_vec == true {
                            new_vec.push(map.get(&s).unwrap().clone());
                        }
                        else {
                            let tmp_vec = map_vec.get(&s).unwrap().clone();
                            if v.is_empty() {
                                v = tmp_vec.clone();
                            }
                            else {
                                for i in 0..v.len() {
                                    v[i] += ope * tmp_vec[i];
                                }
                            }
                        }
                    }
                }
            }
            let out = v.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" ");
            println!("[ {} ]", out);
        },
        _ => unreachable!(),
    }
}