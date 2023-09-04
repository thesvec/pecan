use super::*;

#[derive(Debug)]
pub struct Lexer {
  input:  String,
  pos:    usize,
  tokens: Vec<Token>,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Self {
      input:  input.to_string(),
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

        self.tokens.push(Token::Literal {
          val: Literal::Integer(num.parse::<i64>().unwrap()),
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
            return Err(LexerError::new(start, &format!("Unknown word: {}", word)));
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