use super::*;

#[derive(Debug)]
pub struct Parser {
  tokens:     Vec<Token>,
  pos:        usize,
  curr_start: usize,
  curr_end:   usize,
  program:    Program,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      pos: 0,
      curr_start: 0,
      curr_end: match tokens.get(0) {
        Some(Token::Keyword { end, .. })
        | Some(Token::Identifier { end, .. })
        | Some(Token::Literal { end, .. })
        | Some(Token::Symbol { end, .. })
        | Some(Token::EOF { end, .. }) => *end,
        None => 0,
      },
      program: Program::new(),
      tokens,
    }
  }

  #[allow(dead_code)]
  fn curr(&self) -> &Token {
    if self.pos >= self.tokens.len() {
      return self.tokens.last().unwrap();
    } else {
      return self.tokens.get(self.pos).unwrap();
    }
  }

  #[allow(dead_code)]
  fn next(&mut self) {
    self.pos += 1;
    (self.curr_start, self.curr_end) = match self.curr() {
      Token::Keyword { start, end, .. }
      | Token::Identifier { start, end, .. }
      | Token::Literal { start, end, .. }
      | Token::Symbol { start, end, .. }
      | Token::EOF { start, end } => (*start, *end),
    };
  }

  #[allow(dead_code)]
  fn next_n(&mut self, n: usize) {
    for _ in 0..n {
      self.next();
    }
  }

  #[allow(dead_code)]
  fn peek(&self) -> &Token {
    if self.pos + 1 >= self.tokens.len() {
      return self.tokens.last().unwrap();
    } else {
      return self.tokens.get(self.pos + 1).unwrap();
    }
  }

  #[allow(dead_code)]
  fn peek_n(&self, n: usize) -> &Token {
    if self.pos + n >= self.tokens.len() {
      return self.tokens.last().unwrap();
    } else {
      return self.tokens.get(self.pos + n).unwrap();
    }
  }

  #[allow(dead_code)]
  fn expect_keyword(&mut self, kw: Keyword) -> Result<(), ParserError> {
    match self.curr() {
      Token::Keyword { val, .. } if *val == kw => {
        self.next();
        Ok(())
      },
      Token::Keyword { start, .. }
      | Token::Identifier { start, .. }
      | Token::Literal { start, .. }
      | Token::Symbol { start, .. }
      | Token::EOF { start, .. } => Err(ParserError::new(
        *start,
        &format!(
          "Unexpected {}, expected keyword '{}'",
          self.curr().type_to_string(),
          kw.to_string()
        ),
      )),
    }
  }

  #[allow(dead_code)]
  fn expect_identifier(&mut self) -> Result<String, ParserError> {
    match self.curr() {
      Token::Identifier { val, .. } => {
        let val = val.clone();
        self.next();
        Ok(val)
      },
      Token::Keyword { start, .. }
      | Token::Literal { start, .. }
      | Token::Symbol { start, .. }
      | Token::EOF { start, .. } => Err(ParserError::new(
        *start,
        &format!(
          "Unexpected {}, expected identifier",
          self.curr().type_to_string()
        ),
      )),
    }
  }

  #[allow(dead_code)]
  fn expect_literal(&mut self) -> Result<Literal, ParserError> {
    match self.curr() {
      Token::Literal { val, .. } => {
        let val = val.clone();
        self.next();
        Ok(val)
      },
      Token::Keyword { start, .. }
      | Token::Identifier { start, .. }
      | Token::Symbol { start, .. }
      | Token::EOF { start, .. } => Err(ParserError::new(
        *start,
        &format!(
          "Unexpected {}, expected literal",
          self.curr().type_to_string()
        ),
      )),
    }
  }

  #[allow(dead_code)]
  fn expect_literal_int(&mut self) -> Result<i64, ParserError> {
    match self.expect_literal()? {
      Literal::Integer(i) => Ok(i),
      // _ => Err(ParserError::new(
      //   self.curr_start,
      //   "Expected integer literal",
      // )),
    }
  }

  #[allow(dead_code)]
  fn expect_symbol(&mut self, sym: Symbol) -> Result<(), ParserError> {
    match self.curr() {
      Token::Symbol { val, .. } if *val == sym => {
        self.next();
        Ok(())
      },
      Token::Keyword { start, .. }
      | Token::Identifier { start, .. }
      | Token::Literal { start, .. }
      | Token::Symbol { start, .. }
      | Token::EOF { start, .. } => Err(ParserError::new(
        *start,
        &format!(
          "Unexpected {}, expected symbol '{}'",
          self.curr().type_to_string(),
          sym.to_string()
        ),
      )),
    }
  }

  pub fn parse(&mut self) -> Result<Program, ParserError> {
    while !matches!(self.curr(), Token::EOF { .. }) {
      match self.curr() {
        Token::Keyword { val, .. } => match val {
          Keyword::Exit => {
            self.next();

            self.expect_symbol(Symbol::LeftParen)?;

            let expr = self.expect_literal_int()?;

            self.expect_symbol(Symbol::RightParen)?;

            self
              .program
              .stmts
              .push(StmtNode::Exit(ExprNode::Literal(LiteralNode::Integer(
                expr,
              ))));

            continue;
          },
        },
        Token::Identifier { val, start, .. } => {
          return Err(ParserError::new(
            *start,
            &format!("Unexpected identifier: {}", val),
          ));
        },
        Token::Literal { val, start, .. } => {
          return Err(ParserError::new(
            *start,
            &format!("Unexpected literal: {:?}", val),
          ));
        },
        Token::Symbol { val, start, .. } => {
          return Err(ParserError::new(
            *start,
            &format!("Unexpected symbol: {:?}", val),
          ));
        },
        Token::EOF { start, .. } => {
          return Err(ParserError::new(*start, "Unexpected EOF"));
        },
      }
    }

    Ok(self.program.clone())
  }
}
