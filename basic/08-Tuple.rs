fn main(){
    let yuna:(&str, u8, u8) = ("yuna", 11, 7);
    let sueka = ("sueka", 1, 30);
    println!("{:?}", yuna);
    println!("{:?}", sueka);

    //Destructing
    let (name, month, day) = yuna;
    println!("{}'s birthday is {}/{}", name, month, day);
}
