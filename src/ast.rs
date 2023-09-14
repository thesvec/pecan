use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
  Integer, // 8 bytes
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTableEntry {
  pub name:   String,
  pub ty:     Type,
  pub offset: usize,
}

impl SymbolTableEntry {
  pub fn new(name: String, ty: Type, offset: usize) -> Self {
    Self { name, ty, offset }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
  pub entries: Vec<SymbolTableEntry>,
  pub offset:  usize,
  pub parent:  Option<usize>,
}

impl SymbolTable {
  pub fn new(offset: usize, parent: Option<usize>) -> Self {
    Self {
      entries: Vec::new(),
      offset,
      parent,
    }
  }

  fn size(&self) -> usize {
    let mut size = 0;
    for entry in self.entries.iter() {
      size += match entry.ty {
        Type::Integer => 8,
      }
    }
    size
  }

  fn get(&self, name: &str) -> Option<&SymbolTableEntry> {
    for entry in self.entries.iter() {
      if entry.name == name {
        return Some(entry);
      }
    }
    None
  }

  fn add(&mut self, name: String, ty: Type) {
    let offset = self.offset + self.size();
    self.entries.push(SymbolTableEntry::new(name, ty, offset));
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
  pub stmts:         Vec<Stmt>,
  pub symbol_tables: Vec<SymbolTable>,
  pub curr_table:    usize,
}

impl Program {
  pub fn new() -> Self {
    Self {
      stmts:         Vec::new(),
      symbol_tables: vec![SymbolTable::new(0, None)],
      curr_table:    0,
    }
  }

  pub fn push_stmt(&mut self, stmt: Stmt) {
    self.stmts.push(stmt);
  }

  pub fn push_entry(&mut self, name: String, ty: Type) {
    self.symbol_tables.last_mut().unwrap().add(name, ty);
  }

  pub fn find_entry(&self, name: &str) -> Option<&SymbolTableEntry> {
    let mut table = Some(self.curr_table);
    while let Some(i) = table {
      if let Some(entry) = self.symbol_tables[i].get(name) {
        return Some(entry);
      }
      table = self.symbol_tables[i].parent;
    }
    None
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
