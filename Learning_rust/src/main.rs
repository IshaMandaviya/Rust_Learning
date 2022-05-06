// // function
// fn main() {
//     let a = 20;
//     let b = 20;
//     let c = add(a, b);
//     println!("Hello, world!{}", c);
// }

// fn add(a: i32, b: i32) -> i32 {
//     return a + 10;
// }

// conditional flow
fn main() {
    let a = 10;
    let b = 120;
    let c = 30;
    if a > b {
        if a > c {
            println!("a is big nomber");
        } else {
            println!("c is big number");
        }
    } else {
        if b > c {
            println!("B is big number");
        }
        else{
            println!("c is big number");
        }
    }
}
