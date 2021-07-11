use std::io; //io 라이브러리를 가져옴. io는 std에 속해있어서, std::io에서 가져와야함

fn main(){ //function이고 main이라는 뜻

    let mut x_string = String::new();
    let mut y_string = String::new();
    // let은 변수를 만듬. 기본적으로 변수는 불변.
    // mut 키워드를 사용하면 변수를 가변으로 사용할 수 있음
    // guess는 이제 String의 new()에서 생성된 인스턴스와 동등


    io::stdin().read_line(&mut x_string)
        .expect("fail to read line");
    io::stdin().read_line(&mut y_string)
        .expect("fail to read line");

    let x:i64  = x_string.trim().parse()
        .expect("Please type a integer");
    let y:f64 = y_string.trim().parse()
        .expect("Please type a float");

    println!("{} + {} = {}", x,y,(x as f64)+y);

}