use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml;
use utils::{readln, StringExt};

mod utils;

/// User configuration porter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name to greet
    #[arg(short, long, default_value_t = String::from(""))]
    name: String,
}

#[derive(Deserialize)]
struct Data {
    commands: Commands,
}

#[derive(Deserialize)]
struct Commands {
    name: String,
    command_type: String,
    key: String,
    value: String,
}

fn main() {
    let args = Args::parse();
    let mut name = args.name;
    if name.len() < 1 {
        readln!("Enter name: ", &mut name);
    }
    println!("Hello, {}!", name.capitalize());

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
