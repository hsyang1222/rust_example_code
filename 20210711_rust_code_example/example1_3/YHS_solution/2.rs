use std::io;

fn main(){
    let mut count:u32=0;
    let mut price:u64;
    for n in 0..10{
        let mut insert_value=String::new();
        io::stdin().read_line(&mut insert_value).
            expect("read line fail");
        println!("insert_value is {}", &insert_value);
        price = insert_value.trim().parse::<u64>().
            expect(&insert_value);

        if price >= 50000{
            count = count + 1;
        }
    }

    println!("{}", count);

}
