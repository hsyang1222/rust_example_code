/*2. 희준이는 라코스테 티셔츠와 guess 바지, 아디다스 신발을 갖고 있다. 종찬이는
커버낫 티셔츠를 구매하고 돈이 없어 희준이의 guess 바지와 아디다스 신발을
뺏어갔다. 현재 종찬이가 가지고 있는 티셔츠, 바지, 신발의 브랜드를
출력하라.(디버그 트레잇과 구조체 갱신법을 이용하여 문제를 풀어라.)
*/
//#![allow(unused)]
#[derive(Debug)] //Debug 트레잇 : Debug라 불리는 출력 포맷. 개발자에게 구조체 출력보여줌
struct  Fashion {
    t: String,
    pants: String,
    shoes: String,
}
fn main() {

    let hj = Fashion {
        t: String::from("lacoste"),
        pants: String::from("guess"),
        shoes: String::from("adidas"),
    };

    let jc = Fashion {
        t: String::from("covernat"),
        ..hj
    };

    println!("jc {:?}", jc);
    //hj의 pants와 shoes는 brrowed 되었으므로 값이 없어 실행 오류.
    //뺏어간 것까지 구현됨!!!
    //println!("hj {:?}", hj.t);
    println!("hj's t {:?}", hj.t);
}

