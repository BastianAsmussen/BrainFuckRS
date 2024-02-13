mod args;

use crate::args::Args;
use anyhow::Result;
use clap::Parser as ArgsParser;
use lang::lexer::Lexer;
use lang::parser::Parser;
use lang::util;

fn main() -> Result<()> {
    let args = Args::parse();
    let source = util::files::read_bf_file(&args.file)?;

    let tokens = Lexer::new(&source).lex()?;
    println!("{tokens:#?}");

    let ast = Parser::new(&tokens).parse()?;
    println!("{ast:#?}");

    Ok(())
}
