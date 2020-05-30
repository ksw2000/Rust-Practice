use std::collections::HashMap;

fn main(){
    let mut h = HashMap::new();
    h.insert("台灣", "台北");
    h.insert("美國", "華盛頓");
    h.insert("馬來西亞", "吉隆坡");
    h.insert("韓國", "首爾");
    println!("HashMap len: {}", h.len());
    match h.get(&"台灣"){
        Some(value)=>{
            println!("台灣首都：{}", value);
        }
        None=>{
            println!("nothing found");
        }
    }

    println!();

    //iteration
    for (k, v) in h.iter(){
        println!("h[\"{}\"] = \"{}\"", k, v);
    }

    println!();

    //contains_key
    println!("台灣在 HashMap 裡嗎？ {}", h.contains_key(&"台灣"));

    println!();

    //remove
    h.remove(&"韓國");
    println!("移除韓國後的 HashMap");
    println!("{:?}", h);
}
