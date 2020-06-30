use std::fmt::Display;  // for generic function

struct Data<T> {
    value: T,
}

struct Rectangle<T>{
    width: T,
    height: T,
}

trait Calc{
    fn area(&self) -> i32;
}

impl Calc for Rectangle<i32>{
    fn area(&self) -> i32{
        return self.width * self.height
    }
}

fn print_pro<T:Display>(t:T){
    println!("{}", t);
}

fn main(){
    //Generic Collection
    let mut v : Vec<i32> = vec![8,6,9];
    v.push(4);
    v.push(7);
    println!("{:?}", v);

    //Illustration: Generic Structure
    let t1: Data<i32> = Data{value: 10};
    let t2: Data<String> = Data{value: "Hello".to_string()};
    println!("t1 {}", t1.value);
    println!("t2 {}", t2.value);

    //Traits
    let rec1: Rectangle<i32> = Rectangle{width: 10, height:20};
    println!("rec1's area: {}", rec1.area());

    //Generic Functions
    print_pro(10 as u8);
    print_pro(20 as u16);
    print_pro(30 as i32);
}
