use std::process::ExitCode;

mod error;
pub use error::*;

mod token;
pub use token::*;

mod lexer;
pub use lexer::*;

mod ast;
pub use ast::*;

mod parser;
pub use parser::*;

mod codegen;
pub use codegen::*;

fn main() -> ExitCode {
  let args = std::env::args().collect::<Vec<String>>();

  let file = match args.len() {
    2 => args[1].clone(),
    _ => {
      eprintln!("Usage: {} <file>.pn", args[0]);
      return ExitCode::FAILURE;
    },
  };

  let input = match std::fs::read_to_string(file) {
    Ok(input) => input,
    Err(err) => {
      eprintln!("{}", err.to_string());
      return ExitCode::FAILURE;
    },
  };

  let mut lexer = Lexer::new(&input);

  let tokens = match lexer.lex() {
    Ok(tokens) => {
      // println!("Tokens: {:#?}", tokens);
      tokens
    },
    Err(err) => {
      eprintln!("{}", err.to_string());
      return ExitCode::FAILURE;
    },
  };

  let mut parser = Parser::new(tokens);

  let program = match parser.parse() {
    Ok(program) => {
      // println!("Program: {:#?}", program);
      program
    },
    Err(err) => {
      eprintln!("{}", err.to_string());
      return ExitCode::FAILURE;
    },
  };

  let mut generator = Generator::new(program);

  let output = generator.generate();

  let out_file = "out.asm";
  let obj_file = "out.o";
  let bin_file = "out";

  match std::fs::write(out_file, output) {
    Ok(_) => (),
    Err(err) => {
      eprintln!("{}", err.to_string());
      return ExitCode::FAILURE;
    },
  }

  let status = std::process::Command::new("nasm")
    .arg("-f")
    .arg("elf64")
    .arg("-o")
    .arg(obj_file)
    .arg(out_file)
    .status()
    .expect("Failed to execute nasm");

  if !status.success() {
    eprintln!("Failed to assemble");
    return ExitCode::FAILURE;
  }

  let status = std::process::Command::new("ld")
    .arg("-s")
    .arg("-o")
    .arg(bin_file)
    .arg(obj_file)
    .status()
    .expect("Failed to execute ld");

  if !status.success() {
    eprintln!("Failed to link");
    return ExitCode::FAILURE;
  }

  ExitCode::SUCCESS
}
