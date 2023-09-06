#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
  Exit,
}

impl ToString for Keyword {
  fn to_string(&self) -> String {
    match self {
      Keyword::Exit => "exit".to_string(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Integer(i32),
}

impl ToString for Literal {
  fn to_string(&self) -> String {
    match self {
      Literal::Integer(val) => val.to_string(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
  LeftParen,
  RightParen,
  Equals,
  ColonEquals,
}

impl ToString for Symbol {
  fn to_string(&self) -> String {
    match self {
      Symbol::LeftParen => "(".to_string(),
      Symbol::RightParen => ")".to_string(),
      Symbol::Equals => "=".to_string(),
      Symbol::ColonEquals => ":=".to_string(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Keyword {
    val:   Keyword,
    start: usize,
    end:   usize,
  },
  Identifier {
    val:   String,
    start: usize,
    end:   usize,
  },
  Literal {
    val:   Literal,
    start: usize,
    end:   usize,
  },
  Symbol {
    val:   Symbol,
    start: usize,
    end:   usize,
  },
  Newline {
    start: usize,
    end:   usize,
  },
  EOF {
    start: usize,
    end:   usize,
  },
}

impl Token {
  pub fn type_to_string(&self) -> String {
    match self {
      Token::Keyword { .. } => "Keyword".to_string(),
      Token::Identifier { .. } => "Identifier".to_string(),
      Token::Literal { .. } => "Literal".to_string(),
      Token::Symbol { .. } => "Symbol".to_string(),
      Token::Newline { .. } => "Newline".to_string(),
      Token::EOF { .. } => "EOF".to_string(),
    }
  }

  pub fn start(&self) -> usize {
    match self {
      Token::Keyword { start, .. } => *start,
      Token::Identifier { start, .. } => *start,
      Token::Literal { start, .. } => *start,
      Token::Symbol { start, .. } => *start,
      Token::Newline { start, .. } => *start,
      Token::EOF { start, .. } => *start,
    }
  }

  pub fn end(&self) -> usize {
    match self {
      Token::Keyword { end, .. } => *end,
      Token::Identifier { end, .. } => *end,
      Token::Literal { end, .. } => *end,
      Token::Symbol { end, .. } => *end,
      Token::Newline { end, .. } => *end,
      Token::EOF { end, .. } => *end,
    }
  }
}

impl ToString for Token {
  fn to_string(&self) -> String {
    match self {
      Token::Keyword { val, .. } => val.to_string(),
      Token::Identifier { val, .. } => val.clone(),
      Token::Literal { val, .. } => val.to_string(),
      Token::Symbol { val, .. } => val.to_string(),
      Token::Newline { .. } => "<NEWLINE>".to_string(),
      Token::EOF { .. } => "<EOF>".to_string(),
    }
  }
}
