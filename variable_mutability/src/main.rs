fn main() {
    //1)comst must always annotate the type 
    // Constants can be declared in any scope, including the global scope, 
    //2) which makes them useful for values that many parts of code need to know about
    //3) constants may be set only to a constant expression, 
    // not the result of a value that could only be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("Three hours in seconds {}", THREE_HOURS_IN_SECONDS);
}
