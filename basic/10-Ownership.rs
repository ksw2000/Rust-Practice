/*
Each value in Rust has a variable that is called owner of the value.
Every data stored in Rust will have an owner associated with it.
For example, in the syntax − let age = 30, age is the owner of the value 30.
*/
fn main(){
   let v = vec![1,2,3];
   // vector v owns the object in heap

   //only a single variable owns the heap memory at any given time
   let v2 = v;
   // here two variables owns heap value,
   //two pointers to the same content is not allowed in rust

   //Rust is very smart in terms of memory access ,so it detects a race condition
   //as two variables point to same heap

   //取消註解下面這一行會出錯，因為 vec![1,2,3] 的所有權已經從 v 轉給 v2
   //v已經無權存取 vec![1,2,3] 了
   //println!("{:?}",v);

   println!("{:?}",v2);

   //Passing value to the function
   let u = vec![4,5,6];
   display(u);

   //取消註解下面這一行會出錯，因為 vec![4,5,6] 的所有權已經從 u 轉給函數 display 了
   //u已經無權存取 vec![4,5,6] 了
   //println!("u = {:?}",u);

   //函數回傳時回傳 ownership
   let w = vec![7,8,9];
   let new_w = display_and_retrun(w);
   println!("w = {:?}", new_w);
}

fn display(v:Vec<i32>){
    println!("display() {:?}", v);
}

fn display_and_retrun(v:Vec<i32>) -> Vec<i32>{
    println!("display_and_retrun() {:?}", v);
    v
}
