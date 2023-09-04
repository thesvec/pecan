use super::*;

pub struct Generator {
  program: Program,
  output:  String,
}

impl Generator {
  pub fn new(program: Program) -> Self {
    Self {
      program,
      output: String::new(),
    }
  }

  pub fn generate(&mut self) -> String {
    const SYS_EXIT: u8 = 60;

    self.output.clear();

    let mut data = String::new();
    let mut text = String::new();

    for stmt in self.program.stmts.iter() {
      match stmt {
        Stmt::Exit(expr) => match expr {
          Expr::Literal(literal) => match literal {
            Literal::Integer(val) => {
              text += "  mov rax, ";
              text += &SYS_EXIT.to_string();
              text += "\n";
              text += "  mov rdi, ";
              text += &val.to_string();
              text += "\n";
              text += "  syscall\n";
            },
          },
          Expr::Identifier(ident) => {
            text += "  mov rax, ";
            text += &SYS_EXIT.to_string();
            text += "\n";
            text += "  mov rdi, [";
            text += ident;
            text += "]\n";
            text += "  syscall\n";
          },
        },
        Stmt::VarDecl(ident, expr) => match expr {
          Expr::Literal(literal) => match literal {
            Literal::Integer(val) => {
              data += &format!("{}: dd {}\n", ident, val);
            },
          },
          _ => panic!("Expected literal"),
        },
        Stmt::VarAssign(ident, expr) => match expr {
          Expr::Literal(literal) => match literal {
            Literal::Integer(val) => {
              text += "  mov [";
              text += ident;
              text += "], dword ";
              text += &val.to_string();
              text += "\n";
            },
          },
          Expr::Identifier(ident2) => {
            text += "  mov rax, [";
            text += ident2;
            text += "]\n";
            text += "  mov [";
            text += ident;
            text += "], rax\n";
          },
        },
      }
    }

    self.output += "global _start\n";
    self.output += "section .data\n";
    self.output += &data;
    self.output += "section .text\n";
    self.output += "_start:\n";
    self.output += &text;

    self.output.clone()
  }
}
