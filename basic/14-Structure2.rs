//Method in Structure
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    //Static funntion
    fn count(w:u32, h:u32) -> u32{
        w*h
    }

    //Use static function to create new objects
    fn counstruct(w:u32, h:u32) -> Rectangle{
        return Rectangle{
            width: w,
            height: h
        };
    }
}

fn main(){
    let r = Rectangle{
        width: 10,
        height: 5
    };
    println!("Rectangle w:{} h:{} area:{}", r.width, r.height, r.area());
    println!("use static function: {}", Rectangle::count(10,5));

    let s = Rectangle::counstruct(100,20);
    println!("Rectangle w:{} h:{} area:{}", s.width, s.height, s.area());
}
