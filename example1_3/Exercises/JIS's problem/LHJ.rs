#![allow(unused)]
use std::io;

fn main() {
    let mut cnt: i16 = 0;
    for i in 0..10 {
        let mut fee = String::new();
        io::stdin()
            .read_line(&mut fee)
            .expect("failed");
        let fee: i16 = fee.trim().parse().expect("numberrrr!!!");

        if fee >= 5 {
            cnt = cnt +1;
        } else {}

    }
    println!("hogu number : {}", cnt);
}

