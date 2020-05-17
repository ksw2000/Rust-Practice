fn main(){
    let mut arr: [i32; 3] = [1,2,3];
    change_to_zero(arr);
    println!("change_to_zero(arr):\n{:?}", arr);

    accumulate(&mut arr);
    println!("accumulate(&mut arr):\n{:?}", arr);
}

fn change_to_zero(mut arr:[i32; 3]){
    for i in 0..3{
        arr[i] = 0;
    }
}

fn accumulate(arr:&mut [i32; 3]){
    for i in 1..3{
        arr[i]+=arr[i-1];
    }
}
