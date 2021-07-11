use std::io;

fn main(){
    let mut intsert_n = String::new();
    io::stdin().read_line(&mut intsert_n).
        expect("read fail");
    let int_n:i32 = intsert_n.trim().parse().
        expect("parse fail");

    if int_n <=0 {
        println!("n must be greater than 0");
    }else{
        let mut it_is_prime:bool;

        for i in 2..int_n+1 {
            it_is_prime = true;
            for check in 2..i {
                if i % check == 0 {
                    it_is_prime = false;
                }
            }

            if it_is_prime{
                println!("{}", i);
            }
        }

    }

}
