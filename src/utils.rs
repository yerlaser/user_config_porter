use clap::Parser;
use serde::Deserialize;

#[allow(unused_macros)]
macro_rules! readln {
    ($message: expr, $receiver: expr) => {{
        use std::io::{stdin, stdout, Write};
        print!("{}", $message);
        stdout().flush().expect("Error reading from stdio");
        stdin().read_line($receiver).expect("Error flushing stdio");
        *$receiver = $receiver.trim_end().to_string();
    }};
    ($message: expr, $receiver: expr, $trim_eol: expr) => {{
        use std::io::{stdin, stdout, Write};
        print!("{}", $message);
        stdout().flush().expect("Error reading from stdio");
        stdin().read_line($receiver).expect("Error flushing stdio");
        if $trim_eol {
            *$receiver = $receiver.trim_end().to_string();
        }
    }};
}

// pub(crate) use readln;

pub trait StringExt {
    fn capitalize(&self) -> String;
}

impl StringExt for String {
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

impl StringExt for &str {
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

/// User configuration porter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Configuration file
    #[arg(short, long, default_value_t = String::from("user_config.toml"))]
    pub config_file: String,
}

#[derive(Deserialize)]
pub struct ConfigCommands {
    pub commands: Vec<ConfigCommand>,
}

#[derive(Deserialize)]
pub struct ConfigCommand {
    pub name: String,
    pub command_type: String,
    pub key: String,
    pub value: String,
}
