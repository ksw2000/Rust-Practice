struct Person{
    name:String,
    year:u16,
    month:u8,
    day:u8
}

struct Rectangle{
    width:u32,
    height:u32
}

fn main(){
    let yuna = Person{
        name : String::from("Yuna"),
        year : 2004,
        month : 11,
        day : 7
    };
    println!("{} was born at {}/{:02}/{:02}", yuna.name, yuna.year, yuna.month, yuna.day);

    let mut sueka = Person{
        name : String::from("Sueka"),
        year : 2002,
        month : 0,
        day : 0
    };
    sueka.month = 1;
    sueka.day = 30;

    println!("{} was born at {}/{:02}/{:02}", sueka.name, sueka.year, sueka.month, sueka.day);

    let rec_a = Rectangle{
        width : 10,
        height : 3
    };
    println!("recA width {}, height {}, area {}", rec_a.width, rec_a.height, area(&rec_a));

}

fn area(r:&Rectangle) -> u64{
    (r.width * r.height).into()
}
