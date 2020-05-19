fn main(){
    let v = vec![1,2,3];
    print_vector(&v);
    // Ownership 沒有轉移，只是被 print_vector 借用而已
    // 所以main()裡仍然可以取用
    println!("in main() {:?}", v);

    // Mutable References
    // Int
    let mut num = -10;
    abs(&mut num);
    println!("num = {}", num);

    // String
    let mut str:String = String::from("主なき者よ、夢の杖のもと我の力となれ");
    push_string(&mut str);
    println!("str after push_string(): {}",str);
}

fn print_vector(u:&Vec<i32>){
    println!("in print_vector() {:?}", u);
}

fn abs(a:&mut i32){
    if *a < 0 {
        *a = -(*a);
    }
}

fn push_string(s:&mut String){
    s.push_str("、固着(セキュア)");
}
