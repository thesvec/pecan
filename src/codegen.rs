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

  fn gen_expr(&mut self, expr: Expr) -> String {
    let mut code = String::new();

    match expr {
      Expr::Literal(l) => match l {
        Literal::Integer(i) => {
          code += &format!("  mov rax, {}\n", i);
          code += "  push rax\n";
        },
      },
      Expr::Identifier(i) => {
        let entry = self.program.find_entry(&i).unwrap();
        code += &format!("  mov rax, [rbp - {}]\n", entry.offset);
        code += "  push rax\n";
      },
    }

    code
  }

  fn gen_stmt(&mut self, stmt: Stmt) -> String {
    let mut code = String::new();

    match stmt {
      Stmt::Exit(expr) => {
        code += &self.gen_expr(expr);
        code += "  pop rdi\n";
        code += "  jmp _exit\n";
      },
      Stmt::VarDecl(name, expr) => {
        code += &self.gen_expr(expr);
        code += "  pop rax\n";
        let entry = self.program.find_entry(&name).unwrap();
        code += &format!("  mov [rbp - {}], rax\n", entry.offset);
      },
      Stmt::VarAssign(name, expr) => {
        code += &self.gen_expr(expr);
        code += "  pop rax\n";
        let entry = self.program.find_entry(&name).unwrap();
        code += &format!("  mov [rbp - {}], rax\n", entry.offset);
      },
    }

    code
  }

  pub fn generate(&mut self) -> String {
    self.output.clear();

    self.output += "global _start\n";
    self.output += "section .text\n";
    self.output += "_start:\n";
    self.output += "  mov rbp, rsp\n";

    let stmts = self.program.stmts.clone();
    for stmt in stmts {
      let code = self.gen_stmt(stmt.clone());
      self.output += &code;
    }

    self.output += "  mov rdi, 0\n";
    self.output += "_exit:\n";
    self.output += "  mov rsp, rbp\n";
    self.output += "  mov rax, 60\n";
    self.output += "  syscall\n";

    self.output.clone()
  }
}
