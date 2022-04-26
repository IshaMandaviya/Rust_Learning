// use std::env;
// use std::fs;
// fn main() {
//     let arg:Vec<String>=env::args().collect();
//     let serching_string=&arg[1];
//     let filename=&arg[2];
//     let content=fs::read_to_string(filename).expect("something went wrong");
//     println!("{}",content);
//     // println!("{:?}",&arg);
// }

// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let (query, filename) = parse_config(&args);

//     // --snip--

//     println!("Searching for {}", query);
//     println!("In file {}", filename);

//     let contents = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename)
// }


// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = parse_config(&args);

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.filename);

//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");

//     // --snip--

//     println!("With text:\n{}", contents);
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }


// use std::env;
// use std::fs;
// use std::process;
// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::new(&args).unwrap_or_else(|err|{
//         println!("Peoblem in passing arguments {}",err);
//         process::exit(1);
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.filename);

//     let contents = fs::read_to_string(config.filename)
//         .expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);

//     // --snip--
// }

// // --snip--

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config,&'static str>{
//         if args.len()<3{
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }


use std::env;
use minigreb::Config;
use std::process;

fn main() {
    // --snip--

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e)=minigreb::run(config){
        println!("application has the following error: {}",e);
        process::exit(1);
    };
}

