// Reading from the Console − stdin()
fn main(){
    let mut line = String::new();
    println!("Enter your name:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    // stdin() tries to read all the characters present in the input buffer
    // when it encounters an end-of-line character.
    println!("Hello! {}", line);
    println!("no of bytes read , {}", b1);
}
