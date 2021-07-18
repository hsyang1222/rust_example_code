use std::io;

#[derive(Debug)]
struct Point{
    x : f32,
    y : f32,
}

impl Point{
    fn distance(one:&Point, two:&Point) -> f32{
        ((one.x-two.x).powf(2.) + (one.y - two.y).powf(2.)).sqrt()
    }

    fn clone(&self) -> Point{
        Point{x:self.x, y:self.y}
    }
}

#[derive(Debug)]
struct Tri{
    P1 : Point,
    P2 : Point,
    P3 : Point,
}

impl Tri{
    fn init() -> Tri{
        Tri{P1:Point{x:0., y:0.},
            P2:Point{x:0., y:0.},
            P3:Point{x:0., y:0.}}
    }
    fn area(&self) -> f32 {
        println!("{:?}", &self);
        let P4 = Point{ x:(self.P1.x + self.P2.x)/2.,
                    y:(self.P1.y + self.P2.y)/2.};
        let width:f32 = Point::distance(&self.P1, &self.P2);
        let height:f32 = Point::distance(&self.P3, &P4);
        width * height / 2.
    }

    fn swap(&mut self){
        // make sure distance between p1 and p2 is longest path
        let p1p2 = Point::distance(&self.P1, &self.P2);
        let p2p3 = Point::distance(&self.P3, &self.P2);
        let p1p3 = Point::distance(&self.P1, &self.P3);

        if p1p2 >= p1p3 && p1p2 >= p2p3{
            //longest path is p1-p2

        }else if p1p3 >= p1p2 && p1p3 >= p2p3 {
            println!("set force P1-P2 is longest path. swap (P2, P3)");
            //longest path is p1-p3. swap(p2,p3)
            let temp = self.P2.clone();
            self.P2 = self.P3.clone();
            self.P3 = temp;
        }else{
            println!("set force P1-P2 is longest path. swap (P1, P3)");
            //longest path is p2-p3. swap(p1, p3)
            let temp = self.P1.clone();
            self.P1 = self.P3.clone();
            self.P3 = temp;
        }
    }
}

fn get_point_from_keyboard() -> Point{
    println!("insert x");
    let mut inser_string = String::new();
    io::stdin().read_line(&mut inser_string).expect("get fail");
    let x = inser_string.trim().parse().expect("parse x fail");
    println!("insert y");
    let mut inser_string = String::new();
    io::stdin().read_line(&mut inser_string).expect("get fail");
    let y = inser_string.trim().parse().expect("parse y fail");
    Point{x:x, y:y}
}

fn main(){
    let mut t = Tri::init();

    println!("insert Point1.");
    t.P1 = get_point_from_keyboard();
    println!("insert Point2.");
    t.P2 = get_point_from_keyboard();
    println!("insert Point3.");
    t.P3 = get_point_from_keyboard();

    t.swap();
    println!("area of triangle:{}", t.area());

}
