#[derive(Debug)]
enum Gender{
    Male, Female
}

struct Person{
    name:String,
    sex:Gender
}

enum Operator{
    Add, Sub, Mul, Div
}

fn calc(a:i32, b:i32, o:Operator) -> i32{
    match o{
        Operator::Add =>{
            return a+b;
        }
        Operator::Sub =>{
            return a-b;
        }
        Operator::Mul =>{
            return a*b;
        }
        Operator::Div =>{
            return a/b;
        }
    }
}

fn is_prime(n:i32) -> Option<bool>{
    if n<=1 {
        return None;
    }
    let root = ((n as f32).sqrt()) as i32;
    for i in 2..(root+1){
        if n%i == 0 {
            return Some(false);
        }
    }
    return Some(true);
}

fn print_is_prime(n:i32){
    print!("{} ", n);
    match is_prime(n){
        Some(data) => {
            if data {
                println!("是質數");
            }else{
                println!("是合數");
            }
        }
        None =>{
            println!("不是質數也不是合數");
        }
    }
}

fn main(){
    println!("{:?}", Gender::Male);
    println!("{:?}", Gender::Female);

    let miuna = Person{
        name:String::from("潮留美海"),
        sex:Gender::Female
    };

    let hikari = Person{
        name:String::from("先島光"),
        sex:Gender::Male
    };

    println!("miuna: {} {:?} ", miuna.name, miuna.sex);
    println!("hikari: {} {:?} ", hikari.name, hikari.sex);

    //Option enum
    //Option is a predefined enum in the Rust standard library.
    /*
        enum Option<T> {
           Some(T),      // used to return a value
           None          // used to return null
        }
        Rust 不支援 null
    */
    println!();
    println!();
    println!("is_prime(1) {:?}", is_prime(1));
    println!("is_prime(2) {:?}", is_prime(2));
    println!("is_prime(4) {:?}", is_prime(4));
    println!("is_prime(13) {:?}", is_prime(13));

    //利用 match 抓取 enum
    println!();
    println!();
    println!("calc(9, 4, Operator::Add) = {}", calc(9, 4, Operator::Add));
    println!("calc(9, 4, Operator::Sub) = {}", calc(9, 4, Operator::Sub));
    println!("calc(9, 4, Operator::Mul) = {}", calc(9, 4, Operator::Mul));
    println!("calc(9, 4, Operator::Div) = {}", calc(9, 4, Operator::Div));

    //利用 match 抓取 Option
    println!();
    println!();
    print_is_prime(1);
    print_is_prime(3);
    print_is_prime(4);

    //Match enum with data type
}
