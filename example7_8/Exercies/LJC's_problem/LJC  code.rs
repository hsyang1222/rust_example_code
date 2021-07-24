/*********************************************************************
HashMap에 4명의 이름과 키를 저장해놓고 이름을 입력 받으면 그에 따른 키를 반환해주는 프로그램을 작성하라.
다만, 키는 반올림하여 1의자리까지만 입력하고, 저장되지 않은 이름일 경우 다시 입력 받는 예외처리를 한다.
**********************************************************************/

use std::io;
use std::collections::HashMap;

fn main() {
    let mut members = HashMap::new();
    let mut equl_chk = false;

    members.insert(String::from("양현식"), 181);
    members.insert(String::from("이종찬"), 170);
    members.insert(String::from("이희준"), 175);
    members.insert(String::from("정인수"), 178);


    loop {
        let mut enter_name = String::new();
        println!("enter name");
        io::stdin().read_line(&mut enter_name).expect("fail");
        enter_name = enter_name.trim().to_string();

        for (key, value) in &members {
            if enter_name == *key {
                println!("{}'s height {}", key, value);
                equl_chk = true;
                break;
            }
        }
        if !equl_chk {
            println!("re-enter name");
        }else{
            break;
        }
    }
}
