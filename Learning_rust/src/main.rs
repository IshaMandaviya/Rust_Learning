fn main() {
    let a = 20;
    let b = 20;
    let c = add(a, b);
    println!("Hello, world!{}", c);
}

fn add(a: i32, b: i32) -> i32 {
    return a + 10;
}
