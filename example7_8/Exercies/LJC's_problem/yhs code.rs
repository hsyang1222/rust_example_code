use std::io;
use std::collections::HashMap;

fn get_string_name() -> String{
    let mut string_insert = String::new();
    io::stdin().read_line(&mut string_insert).expect("insert fail");
    String::from(string_insert.trim())
}

fn main() {
    let mut nameheight = HashMap::new();
    nameheight.insert(String::from("yhs"), 164);
    nameheight.insert(String::from("ljc"), 164);
    nameheight.insert(String::from("jis"), 163);
    nameheight.insert(String::from("lhj"), 163);
    loop {
        let name = get_string_name();
        match nameheight.get(&name){
            Some(t) => println!("{}:{}", name, t),
            None => println!("error. there is no {} in hashmap", name)
        }
    }
}
