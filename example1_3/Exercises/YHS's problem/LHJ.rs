/*양현식 코딩 문제
사용자로부터 숫자 n을 입력 받는다. 1~n까지 숫자 중 소수를 모두 찾아 출력하는 프로그램을 작성하시오.
참조 :  https://doc.rust-lang.org/rust-by-example/flow_control/for.html
*/
use std::io;

fn prime(n:i16) -> bool{
    if n == 1 {false}
    else {
        //let nn = n*n;
        for i in 0..n*n {
            if n%i == 0 {false;}
            false
        }
    }
}
fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("failed");

    let n: i16 = n.trim().parse().expect("numberrrr!!!");

    for i in n {
        if prime(n) == true {
            println!("{}", n);
        }
    }

}
