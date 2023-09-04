pub trait Error {
  fn new(pos: usize, msg: &str) -> Self;
  fn pos(&self) -> usize;
  fn msg(&self) -> String;
  fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct LexerError {
  pos: usize,
  msg: String,
}

impl Error for LexerError {
  fn new(pos: usize, msg: &str) -> Self {
    Self {
      pos,
      msg: msg.to_string(),
    }
  }

  fn pos(&self) -> usize {
    self.pos
  }

  fn msg(&self) -> String {
    self.msg.clone()
  }

  fn to_string(&self) -> String {
    format!("Lexer error at position {}: {}", self.pos, self.msg)
  }
}

#[derive(Debug)]
pub struct ParserError {
  pos: usize,
  msg: String,
}

impl Error for ParserError {
  fn new(pos: usize, msg: &str) -> Self {
    Self {
      pos,
      msg: msg.to_string(),
    }
  }

  fn pos(&self) -> usize {
    self.pos
  }

  fn msg(&self) -> String {
    self.msg.clone()
  }

  fn to_string(&self) -> String {
    format!("Parser error at position {}: {}", self.pos, self.msg)
  }
}
