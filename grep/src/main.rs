use std::env; // args
use std::process; // exit

use grep::*;

fn main(){
    let args: Vec<String> = env::args().collect();
     // env::args_os() //return type is OsString use this can get illegal char.

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}


