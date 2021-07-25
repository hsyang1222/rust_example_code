use std::io;
use std::collections::HashMap;

fn get_int() -> i64{
    let mut string_insert = String::new();
    io::stdin().read_line(&mut string_insert).expect("insert fail");
    string_insert.trim().parse().expect("parse fail")
}

fn main() {
    let mut vec = Vec::new();
    let n = get_int();
    for i in 0..n{
        let value = get_int();
        vec.push(value);
    }

    vec.sort();

    for i in vec{
        print!("{} ", i);
    }
}
