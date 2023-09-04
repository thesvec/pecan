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

    self.output.push_str("global _start\n");
    self.output.push_str("section .text\n");
    self.output.push_str("_start:\n");

    for stmt in self.program.stmts.iter() {
      match stmt {
        StmtNode::Exit(expr) => match expr {
          ExprNode::Literal(literal) => match literal {
            LiteralNode::Integer(val) => {
              self.output.push_str(&format!("mov rax, {}\n", SYS_EXIT));
              self.output.push_str(&format!("mov rdi, {}\n", val));
              self.output.push_str("syscall\n");
            },
          },
        },
      }
    }

    self.output.clone()
  }
}
