fn main(){
    //String Literal
    //String literals (&str) are used when the value of a string is known at compile time.

    //String literals are static by default.
    //This means that string literals are guaranteed to
    //be valid for the duration of the entire program.

    let is:&str = "是在";
    let hello:&'static str = "哈囉";
    println!("{}{}", is, hello);

    //String Object
    let str1 = String::new();
    let str2 = String::from("Hello");
    println!("str1's length is {}", str1.len());
    println!("str2's length is {}", str2.len());

    //Common Methods

    //To access all methods of String object
    //convert a string literal to object type using the to_string() function.
    let str3 = "Hello Rust".to_string();
    println!("str3's length is {}", str3.len());

    //push() appends the given char to the end of this String.
    //push_str() function appends a given string slice onto the end of a String.
    let mut str4 = "闇の力を秘めし鍵よ、真の姿を我の前に示せ、契約のもと、さくらが命じる".to_string();
    str4.push('、');
    str4.push_str("レリーズ");
    println!("str4: {}", str4);

    //The trim() function removes leading and trailing spaces in a string.
    //NOTE that this function will not remove the inline spaces.
    let str5 = " 星の力を秘めし鍵よ、真の姿を我の前に示せ、契約のもと、さくらが命じる レリーズ   ".to_string();
    println!("str5: {}", str5.trim());

    println!();
    println!("Example: .chars()");

    //chars()
    let str6 = String::from("レリーズ");
    for c in str6.chars(){
        println!("{}", c);
    }

    println!();

    //Concatenate two String object
    let mut str_dest = String::from("主無きものよ、夢の杖のもと、我の力になれ");
    let str_src  = String::from(" セキュア！");
    str_dest = str_dest + &str_src;
    println!("{}", str_dest);
}
