fn main(){
    //For loop
    //For loop in rust is like for loop in python or foreach in PHP
    //0, 1, 2, ..., 9
    for i in 0..10{
        print!("{} ", i);
    }

    println!();

    //While loop
    let mut i = 0;
    while i<10{
        print!("{} ", i);
        i+=1;
    }

    println!();

    //While(true)
    //loop{}
    let mut j = 0;
    loop{
        print!("{} ", j);
        j+=1;

        if j==10 {
            break;
        }
    }
}
