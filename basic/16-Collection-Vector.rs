fn main(){
    //Vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?} len: {}", v, v.len());

    for i in &v {
        print!("{} ", i);
    }
    println!();

    v.remove(1);
    println!("{:?} ", v);

    println!("Is 3 in v? {}", v.contains(&3));
}
