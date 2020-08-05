// In Rust, errors can be classified into two major categories.
// 1. Recoverable: errors which can be handled
// 2. Unrecoverable: erros which cannot be handled

// A recoverable error is an error that can be corrected.
// A program can retry the failed operation or specify an alternate course of action
// when it encounters a recoverable error.
// Ex: File Not Found error

// Unlike other programming languages, Rust does not have exceptions. It returns
// an enum Result<T, E> for recoverable errors,
// while it calls the `panic macro` if the program encounters an unrecoverable errors.

// panic! macro allows a program to terminate immediately and
// provide feedback to the caller of the program. It sholid be used when
// a program reaches an unrecoverable state.

/*
    fn main(){
        panic!("Hello");
        println!("End of main");    // unrecoverable statement
    }
*/

// Enum Result ─ <T, E> can be used to handle recoverable errors.
// It has two variants Ok and Err. T and E are generic type parameters.
// T represents the type of the value that will be returned in a success case
// within the OK variant
// E represents the type of the error that will be returned in a failure case
// within the Err variant
/*
enum Result<T, E>{
    OK(T),
    ERR(E)
}
*/
use std::fs::File;
fn main(){
    let f = File::open("main.jpg");
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found \n{:?}", e);
        }
    }
    println!("end of main");
}

// The program prints end of the main event though file was not found.
// Too complex
