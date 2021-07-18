/*
2차원 좌표상에 존재하는 삼각형은 그 꼭지점을 (x,y)로 이루어진 좌표 3개로 표현하여 저장할 수 있다.
사용자로부터 (x,y) 좌표 3개를 입력 받고, 이 삼각형의 넓이를 구하는 프로그램을 작성하시오.
단, 구조체 또는 열거형을 이용하여 (x,y)를 저장하시오.
*/

use std::io;
use std::io::Write;

struct StCoord
{
    x: i32,
    y: i32
}

fn main() {
    let mut lb = StCoord{x:i32::MAX, y:i32::MAX};
    let mut rt = StCoord{x:i32::MIN, y:i32::MIN};
    
    for i in 1..4
    {
        print!("Input x{}: ", i);
        io::stdout().flush().unwrap();

        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read line");
        let x: i32 = x.trim().parse().expect("Please input number");

        print!("Input y{}: ", i);
        io::stdout().flush().unwrap();
        
        let mut y = String::new();
        io::stdin().read_line(&mut y).expect("Failed to read line");
        let y: i32 = y.trim().parse().expect("Please input number");

        let t_coord = StCoord{x, y};
        
        if lb.x > t_coord.x
        {
            lb.x = t_coord.x;
        }

        if lb.y > t_coord.y
        {
            lb.y = t_coord.y;
        }

        if rt.x < t_coord.x
        {
            rt.x = t_coord.x;
        }

        if rt.y < t_coord.y
        {
            rt.y = t_coord.y;
        }
    }

    println!("lb({},{})", lb.x, lb.y);
    println!("rt({},{})", rt.x, rt.y);

    let triangle_area = ((rt.x - lb.x) * (rt.y - lb.y)) as f64 / 2.0;
    println!("Triangle Area: {}", triangle_area);
}
