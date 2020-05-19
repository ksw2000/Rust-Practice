fn main(){
    //Slice an int array
    let arr = [1, 2, 4, 8, 16, 32, 64];
    let s   = &arr[0..4];
    println!("{:?}", s);
    println!("slice len {}\n", s.len());

    //Slice a String
    let str = String::from("闇の力を秘めし鍵よ");
    println!("{}", str);
    let s2  = &str[0..9];
    println!("{:?}", s2);

    //Mutable Slices
    let mut arr2 = [10, 20, 30, 40, 50, 60];
    let s3 = &mut arr2[0..3];
    s3[0] = 1010;
    //error: println!("arr2 {:?}", arr2); ownership 已轉移
    println!("{:?}", s3);

    let mut arr3 = [70, 80, 90, 100, 110, 120];
    to_zero(&mut arr3[0..3]);
    println!("{:?}", arr3);
}

fn to_zero(s:&mut [i32]){
    for i in 0..s.len(){
        s[i] = 0;
    }
}
