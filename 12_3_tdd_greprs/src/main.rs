use std::env;
use std::process;
use greprs::my_greprs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args : {:?}\n", args);

    if let Err(e) = GrepConfigs::parse_args(&args).unwrap().read_file() {
        println!("Error catch... : {}", e);
        process::exit(1);
    }
}
