// use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let face='\u{1F600}';
    let face2='\u{1F603}';
    // println!("You guessed: {}", guess);
    println!("{}", face);
    println!("{}", face2);
    println!("{}", face);
    println!("{}", face2);
}
