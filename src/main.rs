use clap::Parser;
use std::fs;
use std::process::{exit, Command};
use toml;
use utils::{Args, ConfigCommand, ConfigCommands};

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

    let config: ConfigCommands = match toml::from_str(&content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not parse file: {config_file}");
            exit(1);
        }
    };

    for command in config.commands {
        let ConfigCommand {
            name,
            command_type,
            key,
            value,
        } = command;

        println!("Name: {name}, Command Type: {command_type}, Key: {key}, Value: {value}");
    }

    let output = Command::new("cat")
        .arg(&config_file)
        .output()
        .expect(&format!("Could not read file: {config_file}"));
    let output = String::from_utf8(output.stdout).unwrap_or_default();
    println!("Original file content is:\n\n{output}");
}
