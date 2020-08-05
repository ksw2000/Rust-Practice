// expect()
// The program can return a custom error message in case of a panic.
// The function expect() is similar to unwrap().
// The only difference is that a custom error message can be displayed using expect.

fn main(){
    let result = is_even(9).expect("...error...");
    //let result = is_even(10).expect("");
    println!("result is {}", result);
    println!("end of main");
}

fn is_even(no: i32) -> Result<bool, String>{
    if no % 2 == 0 {
        return Ok(true);
    }else{
        return Err("NOT_AN_EVEN".to_string());
    }
}
