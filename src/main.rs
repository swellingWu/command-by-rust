// write a command line program
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // the parameter value into the variable
    // let query = &args[1];
    // let filename = &args[2];
    // => to the function of the analytical parameters extracted into a separate function
    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("I file {}", config.filename);
    

    // add the code read the file
    let contents = fs::read_to_string(config.filename).expect("sorry, the file was not found.");// read the file into a character
    println!("With text:\n {}",contents);
}
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{query,filename}
    }
}