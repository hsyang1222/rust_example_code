/*
HashMap에 4명의 이름과 키를 저장해놓고 이름을 입력 받으면 그에 따른 키를 반환해주는 프로그램을 작성하라.
(다만, 키는 반올림하여 1의자리까지만 입력하고, 저장되지 않은 이름일 경우 다시 입력 받는 예외처리를 한다.)
*/

use std::collections::HashMap;
use std::io;

fn main() {
    let mut height_map = HashMap::new();

    height_map.insert(String::from("Lee Jongchan"), 170);
    height_map.insert(String::from("Lee Huijun"), 175);
    height_map.insert(String::from("Yang Hyunsik"), 166);
    height_map.insert(String::from("Jung Insu"), 178);

    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect("Failed to read line");
    let input_name = input_name.trim();

    let height = height_map.get(&input_name.to_string());
    match height {
        Some(i) => println!("{}", i),
        None => {
            let mut input_height = String::new();
            io::stdin().read_line(&mut input_height).expect("Failed to read line");
            let input_height: u32 = input_height.trim().parse().expect("Please input type number");
    
            height_map.insert(input_name.to_string(), input_height);
        }
    }
}
