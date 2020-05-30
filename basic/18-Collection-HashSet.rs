use std::collections::HashSet;

fn main(){
    //HashSet
    //HashSet is a set of unique values of type T. Adding and removing values is fast,
    //and it is fast to ask whether a given value is in the set or not.
    let mut s = HashSet::new();
    s.insert("先島光");
    s.insert("向井戶愛花");
    s.insert("比良平千咲");
    s.insert("伊佐木要");
    s.insert("木原紡");
    s.insert("潮留美海");
    s.insert("久沼紗由");
    println!("HashSet len: {}", s.len());

    //contains
    println!("潮留美海在 HashSet 裡嗎？ {}", s.contains(&"潮留美海"));

    println!();

    //iteration
    print!("來自風平浪静的明天主要角色：");
    for v in s.iter(){
        print!("{} ",v);
    }

    println!();
    println!();

    s.remove(&"伊左木要");
    s.remove(&"久沼沙由");
    s.remove(&"木原紡");
    s.remove(&"比良平千咲");

    println!("移除一些角色後：{:?}", s);
}
