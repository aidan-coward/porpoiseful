extern crate ronky;

use std::env;
use std::process;

use ronky::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Config looks like: {:?}", config);

}