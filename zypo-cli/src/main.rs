//! ## Other Documentation
//!
//! Links to rest of the Zypo (`zypo-rs`) compiler:
//!
//! - [General compiler docs](https://github.com/scOwez/zypo-rs)
//! - [`zypo-lib` - Compiler library documentation](https://docs.rs/zypo-lib)
//!
//! ## About
//!
//! `zypo-cli` is the primary CLI for the Zypo compiler and is what users are
//! most commonly using to compile Zypo code.

use climake::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use zypo_lib::parser::grammar;

/// Exits with error 1 displaying given [String]
fn exit_err(msg: String) -> ! {
    eprintln!(
        "\n## Fatal error\n\nThere was a fatal error when compiling:\n\n```none\n{}\n```",
        msg
    );

    process::exit(1)
}

/// Gets a file path or displays a high-level error for user
fn read_file_or_err(path: PathBuf) -> String {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => exit_err(
            "Could not open given file, please check I have permission to do so".to_string(),
        ),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(_) => exit_err("Could not load given file as utf-8, please check encoding".to_string()),
    }
}

/// Returns the AST for the zypo file given
fn ast_gen(args: Vec<String>) {
    println!("# AST compilation log\n\nThis is the markdown compilation log for Zypo's `--ast` functionality.");

    if args.len() != 1 {
        exit_err("Please supply 1 argument for the zypo file".to_string());
    }

    let file_path = PathBuf::from(args[0].clone());

    if !file_path.exists() {
        exit_err("Please provide a valid file path!".to_string());
    }

    let file_string = read_file_or_err(file_path);
    match grammar::GrammarParser::new().parse(&file_string) {
        Ok(ast) => println!("\n## AST Result\n\nThe following is the found successfully-parsed AST for the given file:\n\n```none\n{:#?}\n```", ast),
        Err(e) => {
            // custom error instead of [error_exit]
            eprintln!("\n## Parsing error\n\nAn error has occured whilst parsing the given Zypo code:\n\n```none\n{:#?}\n```", e);
            process::exit(1);
        }
    };
}

/// Builds the [climake]-based CLI.
fn build_cli() -> CliMake {
    let args = vec![CliArgument::new(
        vec!['a'],
        vec!["ast", "ast-gen"],
        Some("Returns the AST for the zypo file given"),
        Box::new(ast_gen),
    )];

    match CliMake::new(args, Some("Compiler for the Zypo language.")) {
        Ok(cli) => cli,
        Err(_) => exit_err("Could not build CLI, this shouldn't happen!".to_string()),
    }
}

fn main() {
    let mut cli = build_cli();

    cli.parse();
}
