use std::env;
use std::process;

use basic_i_o::Config;

fn main() {
    //NOTE: main() should only be used for setup and handling errors
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else( | err |{
        eprintln!("Problems with parsing args: {}", err);
        process::exit(1);
    });

    println!("Query is: {}", config.query);
    println!("Filename is: {}", config.filename);

    if let Err(e) = basic_i_o::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    };
}


