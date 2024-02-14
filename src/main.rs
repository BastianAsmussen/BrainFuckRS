mod args;

use crate::args::Args;
use anyhow::Result;
use clap::Parser as ArgsParser;
use lang::interpreter::Interpreter;
use lang::lexer::Lexer;
use lang::parser::Parser;
use lang::utils;
use lang::utils::timer;

fn main() -> Result<()> {
    let args = Args::parse();

    let (source, elapsed) = timer::time(|| utils::files::read_bf_file(&args.file));
    let source = source?;
    println!("Read file in {elapsed:?}.");

    let (tokens, elapsed) = timer::time(|| Lexer::new(&source).lex());
    let tokens = tokens?;
    println!("Lexed in {elapsed:?}.");

    let (ast, elapsed) = timer::time(|| Parser::new(&tokens).parse());
    let ast = ast?;
    println!("Parsed in {elapsed:?}.");

    let (_, elapsed) = timer::time(|| Interpreter::new(&ast).run());
    println!("Interpreted in {elapsed:?}.");

    Ok(())
}
