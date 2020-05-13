fn main(){
    //在 rust 中，若沒有用 mut 則皆視為 const
    let num = 5;
    let num2:i32 = 5;
    println!("{} {}",num, num2);

    //num = 7
    //上面那行會報錯
    let mut num3 : i32 = 3;
    println!("{}", num3);
    num3 = 7;
    println!("{}", num3);

    //unsigned int

    //float
    let f = 3e8;
    let f2:f64 = 6.626e-34;
    println!("{} {:e}", f, f2);

    //boolean
    let _b1 = true;
    let _b2:bool = false;

    //charcter
    let _c1 = 'c';
    let _c1:char = 'd';

    //Rust 不提供各類型的隱式轉換
    //要轉型要用 as
    let float_num = 10.56;
    let int_num   = float_num as i32;
    println!("{}", int_num);
}
