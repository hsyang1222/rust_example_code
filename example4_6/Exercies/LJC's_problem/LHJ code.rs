/*금일 먹은 음식이름과 칼로리를 구조체로 저장하여,
전체 칼로리를 출력하는 프로그램을 작성하시오.
(사용자가 입력하지 않아도 무방합니다.)
 */ // 정녕 이종찬은 헬창인가... 칼로리계산이라니...
fn main() {
    struct Helchag {
        food: String,
        calorie: u32,
    }

    let breakfast = Helchag {
        food: String::from("kimbab"),
        calorie: 200,
    };
    let lunch = Helchag {
        food: String::from("lamen"),
        calorie: 400,
    };
    let dinner = Helchag {
        food: String::from("banana"),
        calorie: 80,
    };

    let sum_calorie = breakfast.calorie + lunch.calorie + dinner.calorie;
    println!("sum_calorie : {}", sum_calorie);
}
