mod commands;
mod request;
mod cli;

use std::process::exit;
use rustyline::DefaultEditor;
use crate::commands::Commands;

fn main() {
    let req = request::Request::new("127.0.0.1", 11300);
    if req.is_err() {
        eprintln!("{}", req.err().unwrap());
        exit(-1);
    }

    let mut rl = DefaultEditor::new().unwrap();
    let mut commands_executor = Commands::new(req.unwrap());
    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(input) => {
                let _ = rl.add_history_entry(input.as_str());
                let r = commands_executor.execute(input.as_str());
                if !r {
                    break;
                }
            },
            Err(err) => {
                eprintln!("err: {}", err);
                break;
            }
        }
    }
}