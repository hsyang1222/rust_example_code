/*
희준이는 라코스테 티셔츠와 guess 바지, 아디다스 신발을 갖고 있다.
종찬이는 커버낫 티셔츠를 구매하고 돈이 없어 희준이의 guess 바지와 아디다스 신발을 뺏어갔다.
현재 종찬이가 가지고 있는 티셔츠, 바지, 신발의 브랜드를 출력하라.
(디버그 트레잇과 구조체 갱신법을 이용하여 문제를 풀어라.)
*/

#[derive(Debug)]
struct st_cloth_brand
{
    shirt: String,
    pants: String,
    shoes: String
}

fn main() {
    let huijun = st_cloth_brand {
                        shirt: String::from("lacoste"),
                        pants: String::from("guess"),
                        shoes: String::from("adidas")};
    let jongchan = st_cloth_brand {
                    shirt: String::from("covernat"),
                    ..huijun};

    println!("Jonchan's cloth: {:?}", jongchan);
}
