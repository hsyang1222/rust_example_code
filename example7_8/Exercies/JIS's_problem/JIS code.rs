/*
숫자를 입력받고 입력받은 숫자 개수만큼 정수를 입력받은 후 오름차순 정렬하여 화면에 출력하라.
*/

use std::io;
use std::io::Write;

fn main() {
    let mut vec_nums: Vec<i32> = Vec::new();

    let mut input_size = String::new();
    io::stdin().read_line(&mut input_size).expect("Failed to read line");
    let input_size: u32 = input_size.trim().parse().expect("Please input type number");

    for i in 0..input_size {
        let mut input_num = String::new();
        io::stdin().read_line(&mut input_num).expect("Failed to  read line");
        let input_num: i32 = input_num.trim().parse().expect("Please input type number");

        vec_nums.push(input_num);
    }

    vec_nums.sort();

    for i in &vec_nums {
        print!("{} ", i);
    }

    io::stdout().flush().unwrap();
}
