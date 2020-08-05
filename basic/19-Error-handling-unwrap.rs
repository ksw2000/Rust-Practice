// unwrap()
// Expects self to be Ok/Some and returns the value contained within..
// If it is Err or None instead, it raises a panic with the contents of the error displayed

fn main(){
    let result = is_even(9).unwrap();
    //let result = is_even(10).unwrap();
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
