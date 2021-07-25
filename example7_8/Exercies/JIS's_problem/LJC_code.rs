/*****************************************************************************
숫자를 입력받고 입력받은 숫자 개수만큼 정수를 입력받은 후 오름차순 정렬하여 화면에 출력하라.
******************************************************************************/
use std::io;

fn main() {
    let mut v = Vec::new();
    let mut cnt_num = String::new();
    let mut key_val = 0;
    let mut temp = 0;

    println!("how much enter number");
    io::stdin().read_line(&mut cnt_num).expect("fail");
    let cnt_num = cnt_num.trim().parse().expect("not number");

    for vec_val in (1..cnt_num+1) {
        let mut num1 = String::new();
        println!("enter number {} : ", vec_val);
        io::stdin().read_line(&mut num1).expect("fail");
        let num1 :i32 = match num1.trim().parse() { Ok(num) => num, Err(_) => continue };
        v.push(num1 as usize);
    }

    print!("before sort : ");
    for i in &v {
        print!("{} ", i);
    }
    println!();

    for i in (0..cnt_num) {
        key_val = v[i];
        for j in (i+1..cnt_num) {
            if v[j] < key_val {
                temp = v[j];
                v[j] = key_val;
                key_val = temp;
            }
        }
        v[i] = key_val;
    }

    print!("after sort : ");
    for i in &v {
        print!("{} ", i);
    }
}
