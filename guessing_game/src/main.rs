use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main(){
    println!("Guessing game");
    let secret_no = rand::thread_rng().gen_range(1..101);
    // println!("Secret no is {} ",secret_no);
    
    
    loop{
        println!("Guess your no:");
        let mut guess=String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read line");
        println!("You guess {}",guess);
        let guess:i64= match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match guess.cmp(&secret_no){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("you Win");break;},
        }
    }
}