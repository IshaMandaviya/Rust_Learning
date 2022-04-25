// use std::collections::HashMap;
// fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

// }
// fn main() {
//     use std::collections::HashMap;

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }

// use std::collections::HashMap;
// fn main(){
//     let mut score=HashMap::new();
//     score.insert(String::from("yellow"),10);
//     score.insert(String::from("blue"),20);
//     // let yellowScore=score.get(&String::from("yellow"));
//     // println!("{:?}",&yellowScore);
//     for (key,value) in &score{
//      println!("{}{}",key,value);
//     }

//     let s=score.entry(String::from("yellow"));
//     println!("{:?}",&s);
// }

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}


