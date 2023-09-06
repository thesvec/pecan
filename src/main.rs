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

fn print_help(this: &str) {
  println!(
    "Usage: {} <run option> [options] <file>

Run options:
  -c <file>       Compile to x86_64 linux executable
  -i <file>       Interpret program
  -h, --help      Print this help message

Options:
  -o <out file>   Output file name",
    this
  );
}

fn main() -> ExitCode {
  let args = std::env::args().collect::<Vec<String>>();

  let mut bin_file = "a";

  let file = match args.len() {
    1 | 2 => {
      if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        print_help(&args[0]);
        return ExitCode::SUCCESS;
      } else {
        eprintln!("Usage: {} <run option> [options] <file>", args[0]);
        return ExitCode::FAILURE;
      }
    },
    3 => {
      if args[1] == "-c" {
        args[2].clone()
      } else if args[1] == "-i" {
        eprintln!("Interpret not implemented");
        return ExitCode::FAILURE;
      } else {
        eprintln!("Invalid option: {}", args[1]);
        return ExitCode::FAILURE;
      }
    },
    5 => {
      if args[1] == "-c" {
        if args[2] == "-o" {
          bin_file = &args[3];
          args[4].clone()
        } else {
          eprintln!("Invalid option: {}", args[3]);
          return ExitCode::FAILURE;
        }
      } else {
        eprintln!("Invalid option: {}", args[1]);
        return ExitCode::FAILURE;
      }
    },
    _ => {
      eprintln!("Usage: {} <run option> [options] [file]", args[0]);
      return ExitCode::FAILURE;
    },
  };

  let out_file = &format!("{}.asm", bin_file);
  let obj_file = &format!("{}.o", bin_file);
  let bin_file = &format!("{}.out", bin_file);

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

  let status = std::process::Command::new("rm")
    .arg(out_file)
    .arg(obj_file)
    .status()
    .expect("Failed to execute rm");

  if !status.success() {
    eprintln!("Failed to remove temporary files");
    return ExitCode::FAILURE;
  }

  ExitCode::SUCCESS
}
