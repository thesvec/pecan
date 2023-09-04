use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Integer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
  pub stmts: Vec<Stmt>,
  pub vars:  HashMap<String, Type>,
}

impl Program {
  pub fn new() -> Self {
    Self {
      stmts: Vec::new(),
      vars:  HashMap::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  Exit(Expr),
  VarDecl(String, Expr),
  VarAssign(String, Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Literal(Literal),
  Identifier(String),
}

impl ToString for Expr {
  fn to_string(&self) -> String {
    match self {
      Expr::Literal(literal) => literal.to_string(),
      Expr::Identifier(ident) => ident.to_string(),
    }
  }
}
