use std::io;

#[derive(Debug)]
struct food{
    name : String,
    cal : f32,
}

struct menu{
    food1 : Option<food>,
    food2 : Option<food>,
    food3 : Option<food>,
    food4 : Option<food>,
    food5 : Option<food>,
}

impl menu{
    fn get_total_cal(&self) ->f32{
        let f1c = match &self.food1{
            Some(P) => P.cal,
            None => 0.,
        };
        let f2c = match &self.food2{
            Some(P) => P.cal,
            None => 0.,
        };
        let f3c = match &self.food3{
            Some(P) => P.cal,
            None => 0.,
        };
        let f4c = match &self.food4{
            Some(P) => P.cal,
            None => 0.,
        };
        let f5c = match &self.food5{
            Some(P) => P.cal,
            None => 0.,
        };


        f1c+f2c+f3c+f4c+f5c
    }
}

fn main(){
    let lhj:menu = menu{food1: Option::from(food { name: String::from("편의점도시락"), cal: 400. }),
                        food2:Option::from(food{name:String::from("편의점도시락"),cal:400.}),
                        food3:Option::from(food{name:String::from("편의점도시락"),cal:400.}),
                        food4:None,
                        food5:None};
    println!("total cal is {}", lhj.get_total_cal());
}