use std::io;
use std::collections::HashMap;

fn insert_key() -> String {
    let mut string_key = String::new();
    io::stdin().read_line(&mut string_key).expect("parse fail");
    let value = string_key.trim();
    String::from(value)
}

fn main() {
    let mut wordcount = HashMap::new();
    loop{
        let string_key=  insert_key();

        if string_key == "end"{
            break
        }
        if wordcount.contains_key(&string_key){
            let original_count = wordcount.get(&string_key);
            let int_count = match original_count {
                None => 0,
                Some(t) => t+1,
            };
            wordcount.insert(string_key, int_count);
        }else{
            wordcount.insert(string_key, 1);
        }
    }
    for (key, value) in &wordcount{
        println!("{}:{}", key, value);
    }
}
