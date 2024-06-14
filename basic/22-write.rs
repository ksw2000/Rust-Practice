use std::io::Write;

fn main(){
    // write(&buf)->Result
    // It returns an io::Result, the number of bytes written
    let b1 = std::io::stdout().write("Hello ".as_bytes()).unwrap();
    let b2 = std::io::stdout().write(String::from("world").as_bytes()).unwrap();
    std::io::stdout().write(format!("\nbytes written {}", b1+b2).as_bytes()).unwrap();
}
