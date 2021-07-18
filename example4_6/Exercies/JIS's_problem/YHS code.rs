#[derive(Debug)]
struct Human{
    name : String,
    tshirt : Option<String>,
    pants : Option<String>,
    shoes : Option<String>,
}

impl Human{
    fn steal(&mut self, other:&mut Human){
        self.pants = other.pants.clone();
        other.pants = None;

        self.shoes = other.shoes.clone();
        other.shoes = None;
    }

}

fn main(){
    let mut leehj = Human{   name:String::from("Lee hee-joon"),
        tshirt: Some(String::from("lacoste")),
        pants:  Some(String::from("guess")),
        shoes:  Some(String::from("adidas"))};

    let mut leejc = Human{  name:String::from("Lee jong-chan"),
        tshirt: Some(String::from("covernat")),
        pants:  None,
        shoes:  None};

    leejc.steal(&mut leehj);
    println!("{:?}", leejc);
    println!("{:?}", leehj);
}