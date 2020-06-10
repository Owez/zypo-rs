//! # Other Documentation
//!
//! Links to rest of the Zypo (`zypo-rs`) compiler:
//!
//! - [General compiler docs](https://github.com/scOwez/zypo-rs)
//! - [`zypo-lib` - Compiler library documentation](https://docs.rs/zypo-lib)
//!
//! # About
//!
//! `zypo-cli` is the primary CLI for the Zypo compiler and is what users are
//! most commonly using to compile Zypo code.

use climake::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;
use zypo_lib::parser::ast_result;

/// Exits with error 1 displaying given &[str]
fn exit_err(msg: &str) -> ! {
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
        Err(_) => exit_err("Could not open given file, please check I have permission to do so"),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(_) => exit_err("Could not load given file as utf-8, please check encoding"),
    }
}

/// Returns the AST for the zypo file given
fn ast_gen(args: Vec<String>) {
    println!("# Zypo compile log\n\nThis is the markdown log for Zypo updated in realtime when compiling.");

    if args.len() != 1 {
        exit_err("Please supply 1 argument for the zypo file");
    }

    let file_path = PathBuf::from(args[0].clone());

    if !file_path.exists() {
        exit_err("Please provide a valid file path!");
    }

    let file_string = read_file_or_err(file_path);
    let ast = ast_result(&file_string);

    println!("\n## AST Result\n\nThe following is the found successfully-parsed AST for the given file:\n\n```none\n{:#?}\n```", ast);
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
        Err(_) => exit_err("Could not build CLI, this shouldn't happen!"),
    }
}

fn main() {
    let mut cli = build_cli();

    cli.parse();
}
