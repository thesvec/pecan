use super::*;

#[derive(Debug)]
pub struct Lexer {
  input:  Box<str>,
  pos:    usize,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Self {
      input:  input.into(),
      pos:    0,
      tokens: Vec::new(),
    }
  }

  #[allow(dead_code)]
  fn pos(&self) -> usize {
    self.pos
  }

  #[allow(dead_code)]
  fn curr(&self) -> Option<char> {
    self.input.chars().nth(self.pos)
  }

  #[allow(dead_code)]
  fn next(&mut self) -> Option<char> {
    self.pos += 1;
    self.curr()
  }

  #[allow(dead_code)]
  fn next_n(&mut self, n: usize) -> Option<char> {
    self.pos += n;
    self.curr()
  }

  #[allow(dead_code)]
  fn peek(&self) -> Option<char> {
    self.input.chars().nth(self.pos + 1)
  }

  #[allow(dead_code)]
  fn peek_n(&self, n: usize) -> Option<char> {
    self.input.chars().nth(self.pos + n)
  }

  pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
    while let Some(c) = self.curr() {
      if c.is_ascii_whitespace() {
        if c == '\n' {
          self.tokens.push(Token::Newline {
            start: self.pos,
            end:   self.pos + 1,
          });
        }
        self.next();
      } else if c.is_ascii_digit() {
        let mut num = String::new();
        let start = self.pos;

        while let Some(c) = self.curr() {
          if c.is_ascii_digit() {
            num.push(c);
            self.next();
          } else {
            break;
          }
        }

        if let Some(c) = self.curr() {
          if c.is_ascii_alphabetic() || c == '_' {
            return Err(LexerError::new(
              self.pos,
              "Expected whitespace or symbol after number",
            ));
          }
        }

        self.tokens.push(Token::Literal {
          val: Literal::Integer(num.parse::<i32>().unwrap()),
          start,
          end: self.pos,
        });
      } else if c.is_ascii_alphabetic() || c == '_' {
        let mut word = String::new();
        let start = self.pos;

        while let Some(c) = self.curr() {
          if c.is_ascii_alphanumeric() || c == '_' {
            word.push(c);
            self.next();
          } else {
            break;
          }
        }

        match word.as_str() {
          "exit" => {
            self.tokens.push(Token::Keyword {
              val: Keyword::Exit,
              start,
              end: self.pos,
            });
          },
          _ => {
            self.tokens.push(Token::Identifier {
              val: word,
              start,
              end: self.pos,
            });
          },
        }
      } else if c == '(' {
        let start = self.pos;

        self.next();

        self.tokens.push(Token::Symbol {
          val: Symbol::LeftParen,
          start,
          end: self.pos,
        });
      } else if c == ')' {
        let start = self.pos;

        self.next();

        self.tokens.push(Token::Symbol {
          val: Symbol::RightParen,
          start,
          end: self.pos,
        });
      } else if c == ':' {
        let start = self.pos;

        if !matches!(self.peek(), Some('=')) {
          return Err(LexerError::new(self.pos, "Expected '=' after ':'"));
        }

        self.next_n(2);

        self.tokens.push(Token::Symbol {
          val: Symbol::ColonEquals,
          start,
          end: self.pos,
        });
      } else if c == '=' {
        let start = self.pos;

        self.next();

        self.tokens.push(Token::Symbol {
          val: Symbol::Equals,
          start,
          end: self.pos,
        });
      } else {
        return Err(LexerError::new(
          self.pos,
          &format!("Unknown character: {}", c),
        ));
      }
    }

    self.tokens.push(Token::EOF {
      start: self.pos,
      end:   self.pos,
    });

    Ok(self.tokens.clone())
  }
}
