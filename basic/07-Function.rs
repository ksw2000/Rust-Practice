fn hello(){
    println!("是在哈囉！");
}

fn add(a:i32, b:i32) -> i32{
    return a+b;
}

fn sub(a:i32, b:i32) -> i32{
    a-b //no semicolon means this value is returned
}

fn add_to(dest:&mut i32, src:i32){
    *dest += src;
}

fn main(){
    hello();
    let mut a:i32 = 10;
    let b:i32 = 20;
    println!("add({}, {}) = {}", a, b, add(a,b));
    println!("sub({}, {}) = {}", a, b, sub(a,b));
    println!("addTo({}, {})", a, b);
    add_to(&mut a, b);
    println!("{}", a);
}
