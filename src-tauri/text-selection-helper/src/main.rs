use clap::{command, Parser};
use serde::Serialize;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

#[derive(Debug, Serialize, Default)]
struct CliResult {
    selected_text: Option<String>,
    err: Option<String>,
}

fn main() {
    let _args = Args::parse();

    text_selection::init();

    let mut cli_result: CliResult = Default::default();

    let text = text_selection::get_selected_text();

    match text {
        Ok(text) => {
            cli_result.selected_text = Some(text);
        }
        Err(err) => {
            cli_result.err = Some(err.to_string());
        }
    }

    let json = serde_json::to_string(&cli_result).unwrap();
    println!("{}", json);
}
