// write a command line program
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // the parameter value into the variable
    // let query = &args[1];
    // let filename = &args[2];
    // => to the function of the analytical parameters extracted into a separate function
    let config = Config::new(&args).unwrap_or_else(|err| {
        // unwrap_or_else -> perform some custom and won't produce panic!
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    };
}