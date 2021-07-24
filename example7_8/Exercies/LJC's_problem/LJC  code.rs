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
