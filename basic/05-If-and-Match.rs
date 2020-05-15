fn main(){
    // 基礎用法，跟 GO 類似
    let a = 10;
    if a == 10 {
        println!("a == 10");
    }else{
        println!("a != 10");
    }

    // 類似三元運算
    let b = if a == 10 {1} else {0};
    println!("b is {}", b);

    //Match Statement
    let num = "two";
    let c = match num{
        "one" => 1,
        "two" => 2,
        "three" => 3,
        _ => 0
    };
    println!("c is {}", c);
}
