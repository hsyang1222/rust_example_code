/*Q2.2차원 좌표상에 존재하는 삼각형은 그 꼭지점을 (x,y)로 이루어진 좌표 3개로
표현하여 저장할 수 있다. 사용자로부터 (x,y) 좌표 3개를 입력 받고, 이 삼각형의
넓이를 구하는 프로그램을 작성하시오. 단, 구조체 또는 열거형을 이용하여 (x,y)를
저장하시오.
*/
// a(a1,a2) b(b1,b2) c(c1,c2) / s= 1/2{(a1-c1)*(b2-c2)-(a2-c2)*(b1-c1)}
use std::io;
#[derive(Debug)]

struct Location {
    x: i32,
    y: i32,
}

fn main() {
    let mut a1 = String::new();
    let mut a2 = String::new();
    let mut b1 = String::new();
    let mut b2 = String::new();
    let mut c1 = String::new();
    let mut c2 = String::new();

    io::stdin().read_line( & mut a1)
        .expect("Failed");
    io::stdin().read_line( & mut a2)
        .expect("Failed");
    let a = Location {
        x: a1.trim().parse()
            .expect("num!!"),
        y: a2.trim().parse()
            .expect("num!!"),
    };

    io::stdin().read_line( & mut b1)
        .expect("Failed");
    io::stdin().read_line( & mut b2)
        .expect("Failed");
    let b = Location {
        x: b1.trim().parse()
            .expect("Number!!"),
        y: b2.trim().parse()
            .expect("num!!")
    };

    io::stdin().read_line( & mut c1)
        .expect("Failed");
    io::stdin().read_line( & mut c2)
        .expect("Failed");
    let c = Location {
        x: c1.trim().parse()
            .expect("Number!!"),
        y: c2.trim().parse()
            .expect("num!!"),
    };

    let s = ((a.x-c.x)*(b.y-c.y)-(a.y-c.y)*(b.x-c.x))/2;
    if s > 0
        {println!("dimentions {:?}", s);}
    else
        {println!("dimentions {:?}", -s);}

}
