fn main(){
    // Array [T; N]
    let mut array: [i32; 3] = [0; 3];
    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);

    array[1] = 1;
    array[2] = 2;

    println!("\nfor x in &array");

    for x in &array{
        println!("{}", x);
    }

    // Vec 動態
    let mut _v1: Vec<i32> = Vec::new();
    let _v2: Vec<i32> = vec![];
    let _v3 = vec![1, 2, 3, 4, 5];

    _v1.push(1);
    _v1.push(2);
    println!("_v1.pop() = {:?}", _v1.pop());
    println!("_v1.pop() = {:?}", _v1.pop());
}
