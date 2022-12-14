use clap::Parser;
use std::fs;
use std::process::exit;
use toml;
use utils::{readln, Args, Commands, Data, StringExt};

mod utils;

fn main() {
    let args = Args::parse();
    let mut name = args.name;
    if name.len() < 1 {
        readln!("Enter name: ", &mut name);
    }
    let name = name.capitalize();
    println!("Hello, {name}!");

    let filename = "user_config.toml";
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file: {filename}");
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not parse file: {filename}");
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
