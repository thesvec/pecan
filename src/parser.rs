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
      curr_end: tokens.get(0).unwrap().end(),
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
      t => Err(ParserError::new(
        t.start(),
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
      t => Err(ParserError::new(
        t.start(),
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
      t => Err(ParserError::new(
        t.start(),
        &format!(
          "Unexpected {}, expected literal",
          self.curr().type_to_string()
        ),
      )),
    }
  }

  #[allow(dead_code)]
  fn expect_literal_int(&mut self) -> Result<i32, ParserError> {
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
      t => Err(ParserError::new(
        t.start(),
        &format!(
          "Unexpected {}, expected symbol '{}'",
          self.curr().type_to_string(),
          sym.to_string()
        ),
      )),
    }
  }

  fn parse_expr(&mut self) -> Result<Expr, ParserError> {
    match self.curr() {
      Token::Literal { .. } => {
        let literal = self.expect_literal()?;
        Ok(Expr::Literal(literal))
      },
      Token::Identifier { .. } => {
        let ident = self.expect_identifier()?;

        if !self.program.vars.contains_key(&ident) {
          return Err(ParserError::new(
            self.tokens.get(self.pos - 1).unwrap().start(),
            &format!("Variable '{}' not declared", ident),
          ));
        }

        Ok(Expr::Identifier(ident))
      },
      t => Err(ParserError::new(
        t.start(),
        &format!(
          "Unexpected {}, expected expression",
          self.curr().type_to_string()
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

            let expr = self.parse_expr()?;

            self.expect_symbol(Symbol::RightParen)?;

            self.program.stmts.push(Stmt::Exit(expr));
          },
        },
        Token::Identifier { val, .. } => {
          let id = val.clone();

          self.next();

          match self.curr() {
            Token::Symbol { val, .. } if *val == Symbol::ColonEquals => {
              self.next();

              let expr = self.parse_expr()?;

              if self.program.vars.contains_key(&id) {
                return Err(ParserError::new(
                  self.tokens.get(self.pos - 1).unwrap().start(),
                  &format!("Variable '{}' already declared", id),
                ));
              }

              self.program.stmts.push(Stmt::VarDecl(id.clone(), expr));

              self.program.vars.insert(id, Type::Integer);
            },
            Token::Symbol { val, .. } if *val == Symbol::Equals => {
              self.next();

              let expr = self.parse_expr()?;

              if !self.program.vars.contains_key(&id) {
                return Err(ParserError::new(
                  self.tokens.get(self.pos - 1).unwrap().start(),
                  &format!("Variable '{}' not declared", id),
                ));
              }

              self.program.stmts.push(Stmt::VarAssign(id, expr));
            },
            _ => {
              return Err(ParserError::new(
                self.tokens.get(self.pos - 1).unwrap().start(),
                &format!("Unexpected token: {:?}", self.curr()),
              ));
            },
          }
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
