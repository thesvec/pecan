use super::*;

#[derive(Debug)]
pub struct Parser {
  tokens:  Vec<Token>,
  pos:     usize,
  program: Program,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      pos: 0,
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
  }

  #[allow(dead_code)]
  fn next_n(&mut self, n: usize) {
    self.pos += n;
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
  fn skip_newlines(&mut self) {
    while matches!(self.curr(), Token::Newline { .. }) {
      self.next();
    }
  }

  #[allow(dead_code)]
  fn expect_keyword(&mut self, kw: Keyword) -> Result<(), ParserError> {
    loop {
      match self.curr() {
        Token::Newline { .. } => {
          self.next();
          continue;
        },
        Token::Keyword { val, .. } if *val == kw => {
          self.next();
          return Ok(());
        },
        t => {
          return Err(ParserError::new(
            t.start(),
            &format!(
              "Unexpected {}, expected '{}'",
              self.curr().type_to_string(),
              kw.to_string()
            ),
          ))
        },
      }
    }
  }

  #[allow(dead_code)]
  fn expect_identifier(&mut self) -> Result<String, ParserError> {
    loop {
      match self.curr() {
        Token::Newline { .. } => {
          self.next();
          continue;
        },
        Token::Identifier { val, .. } => {
          let val = val.clone();
          self.next();
          return Ok(val);
        },
        t => {
          return Err(ParserError::new(
            t.start(),
            &format!(
              "Unexpected {}, expected identifier",
              self.curr().type_to_string()
            ),
          ))
        },
      }
    }
  }

  #[allow(dead_code)]
  fn expect_literal(&mut self) -> Result<Literal, ParserError> {
    loop {
      match self.curr() {
        Token::Newline { .. } => {
          self.next();
          continue;
        },
        Token::Literal { val, .. } => {
          let val = val.clone();
          self.next();
          return Ok(val);
        },
        t => {
          return Err(ParserError::new(
            t.start(),
            &format!(
              "Unexpected {}, expected literal",
              self.curr().type_to_string()
            ),
          ))
        },
      }
    }
  }

  #[allow(dead_code)]
  fn expect_literal_int(&mut self) -> Result<i32, ParserError> {
    match self.expect_literal()? {
      Literal::Integer(i) => Ok(i),
    }
  }

  #[allow(dead_code)]
  fn expect_symbol(&mut self, sym: Symbol) -> Result<(), ParserError> {
    loop {
      match self.curr() {
        Token::Newline { .. } => {
          self.next();
          continue;
        },
        Token::Symbol { val, .. } if *val == sym => {
          self.next();
          return Ok(());
        },
        t => {
          return Err(ParserError::new(
            t.start(),
            &format!(
              "Unexpected {}, expected symbol '{}'",
              self.curr().type_to_string(),
              sym.to_string()
            ),
          ))
        },
      }
    }
  }

  fn type_of(&self, expr: &Expr) -> Type {
    match expr {
      Expr::Literal(lit) => match lit {
        Literal::Integer(_) => Type::Integer,
      },
      Expr::Identifier(ident) => {
        todo!("type_of: {}", ident);
      },
    }
  }

  fn parse_expr(&mut self) -> Result<Expr, ParserError> {
    loop {
      match self.curr() {
        Token::Newline { .. } => {
          self.next();
          continue;
        },
        Token::Literal { .. } => {
          let literal = self.expect_literal()?;
          return Ok(Expr::Literal(literal));
        },
        Token::Identifier { .. } => {
          let ident = self.expect_identifier()?;

          if let None = self.program.find_entry(&ident, false) {
            return Err(ParserError::new(
              self.tokens.get(self.pos - 1).unwrap().start(),
              &format!("Variable '{}' not declared", ident),
            ));
          }

          return Ok(Expr::Identifier(ident));
        },
        t => {
          return Err(ParserError::new(
            t.start(),
            &format!(
              "Unexpected {}, expected expression",
              self.curr().type_to_string()
            ),
          ))
        },
      }
    }
  }

  fn parse_stmt(&mut self) -> Result<Option<Stmt>, ParserError> {
    fn _parse_exit(parser: &mut Parser) -> Result<Stmt, ParserError> {
      parser.expect_keyword(Keyword::Exit)?;

      parser.expect_symbol(Symbol::LeftParen)?;

      let expr = parser.parse_expr()?;

      parser.expect_symbol(Symbol::RightParen)?;

      Ok(Stmt::Exit(expr))
    }

    fn _parse_var_decl(parser: &mut Parser, ident: &str) -> Result<Stmt, ParserError> {
      parser.expect_symbol(Symbol::ColonEquals)?;

      let expr = parser.parse_expr()?;

      if let Some(_) = parser.program.find_entry(ident, true) {
        return Err(ParserError::new(
          parser.tokens.get(parser.pos - 1).unwrap().start(),
          &format!("Variable '{}' already declared in this scope", ident),
        ));
      }

      parser
        .program
        .push_entry(ident.to_string(), parser.type_of(&expr));

      Ok(Stmt::VarDecl(ident.to_string(), expr))
    }

    fn _parse_var_assign(parser: &mut Parser, ident: &str) -> Result<Stmt, ParserError> {
      parser.expect_symbol(Symbol::Equals)?;

      let expr = parser.parse_expr()?;

      if let None = parser.program.find_entry(ident, false) {
        return Err(ParserError::new(
          parser.tokens.get(parser.pos - 1).unwrap().start(),
          &format!("Variable '{}' not declared", ident),
        ));
      }

      Ok(Stmt::VarAssign(ident.to_string(), expr))
    }

    let stmt;

    loop {
      match self.curr() {
        Token::EOF { .. } => return Ok(None),
        Token::Newline { .. } => {
          self.next();
          continue;
        },
        Token::Keyword { val, .. } => match val {
          Keyword::Exit => {
            stmt = _parse_exit(self)?;

            break;
          },
        },
        Token::Identifier { val, .. } => {
          let ident = val.clone();

          self.next();
          self.skip_newlines();

          stmt = match self.curr() {
            Token::Symbol { val, .. } => match val {
              Symbol::ColonEquals => _parse_var_decl(self, &ident)?,
              Symbol::Equals => _parse_var_assign(self, &ident)?,
              Symbol::LeftParen => todo!("function calls not implemented"),
              _ => {
                return Err(ParserError::new(
                  self.curr().start(),
                  &format!("Unexpected {}", self.curr().type_to_string()),
                ))
              },
            },
            t => {
              return Err(ParserError::new(
                t.start(),
                &format!(
                  "Unexpected {}, expected symbol",
                  self.curr().type_to_string()
                ),
              ))
            },
          };

          break;
        },
        t => {
          return Err(ParserError::new(
            t.start(),
            &format!(
              "Unexpected {}, expected statement",
              self.curr().type_to_string()
            ),
          ))
        },
      }
    }

    if let Token::Newline { .. } = self.curr() {
      self.next();
      self.skip_newlines();
    } else {
      return Err(ParserError::new(
        self.curr().start(),
        &format!(
          "Unexpected {}, expected newline or EOF after statement",
          self.curr().type_to_string()
        ),
      ));
    }

    Ok(Some(stmt))
  }

  pub fn parse(&mut self) -> Result<Program, ParserError> {
    while !matches!(self.curr(), Token::EOF { .. }) {
      let stmt = self.parse_stmt()?;

      if stmt.is_some() {
        self.program.push_stmt(stmt.unwrap());
      }
    }

    Ok(self.program.clone())
  }
}
