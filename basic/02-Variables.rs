fn main(){
    // immutable
    // When a variable is immutable, 
    // once a value is bound to a name, 
    // you can’t change that value.
    let num = 5;
    let num2:i32 = 5;
    println!("{} {}",num, num2);
    
    // num = 7
    // Error: cannot assign twice to immutable variable

    let mut num3 : i32 = 3;
    println!("{}", num3);
    num3 = 7;
    println!("{}", num3);

    // unsigned int
    let u_num:u32 = 10;
    println!("{}", u_num);

    // float
    let f = 3e8;
    let f2:f64 = 6.626e-34;
    println!("{} {:e}", f, f2);

    // boolean
    let _b1 = true;
    let _b2:bool = false;

    // character (4 bytes)
    // use Unicode
    let _c1 = 'c';
    let _c1:char = 'd';

    let float_num = 10.56;
    let int_num   = float_num as i32;
    println!("{}", int_num);

    // shadowing
    // you can declare a new variable with the same name 
    // as a previous variable.
    // In effect, the second variable overshadows the first, taking 
    // any uses of the variable name to itself until either it itself is shadowed or the scope ends.
    let x = 5;
    let x = x + 1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {}",x);
        // The value of x in the inner scope is: 12
    }
    println!("The value of x is: {}",x);
    // The value of x is: 6 

    let _spaces = "       ";
    let _spaces = _spaces.len();
}
