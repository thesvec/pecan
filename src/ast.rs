#[derive(Debug, Clone, PartialEq)]
pub struct Program {
  pub stmts: Vec<StmtNode>,
}

impl Program {
  pub fn new() -> Self {
    Self { stmts: Vec::new() }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtNode {
  Exit(ExprNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprNode {
  Literal(LiteralNode),
}

impl ToString for ExprNode {
  fn to_string(&self) -> String {
    match self {
      ExprNode::Literal(literal) => literal.to_string(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralNode {
  Integer(i64),
}

impl ToString for LiteralNode {
  fn to_string(&self) -> String {
    match self {
      LiteralNode::Integer(val) => val.to_string(),
    }
  }
}
