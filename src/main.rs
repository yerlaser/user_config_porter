use clap::Parser;
use std::fs;
use std::process::exit;
use toml;
use utils::{Args, Commands, Data};

mod utils;

fn main() {
    let args = Args::parse();
    let config_file = args.config_file;
    let content = match fs::read_to_string(&config_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file: {config_file}");
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not parse file: {config_file}");
            exit(1);
        }
    };
    let Commands {
        name,
        command_type,
        key,
        value,
    } = data.commands;
    println!("Name: {name}, Command Type: {command_type}, Key: {key}, Value: {value}");
}
