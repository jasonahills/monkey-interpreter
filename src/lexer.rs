use std::iter::*;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
  Illegal,
  EOF,

  Ident(String),
  Int(u32),

  Assign,
  Plus,
  Minus,
  Asterisk,
  Slash,
  GT,
  LT,
  Comma,
  Semicolon,
  LParen,
  RParen,
  LBrace,
  RBrace,
  Eq,
  NotEq,

  Function,
  Let,
  True,
  False,
  If,
  Else,
  Return,
}

type CharTest = fn(&char) -> bool;

pub struct Lexer<'a> {
  chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Lexer {
      chars: input.chars().peekable(),
    }
  }

  fn accumulate_while(&mut self, test: CharTest, start_with: char) -> String {
    let mut acc = vec!(start_with);
    while let Some(peek_c) = self.chars.peek() {
      if !test(peek_c) {
        break;
      }
      acc.push(*peek_c);
      self.chars.next();
    }
    acc.iter().collect()
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    let mut c = self.chars.next()?;

    // eat whitespace
    while is_monkey_whitespace(&c) {
      c = self.chars.next()?;
    }
    let c = c;

    match c {
      ';' => Some(Token::Semicolon),
      '(' => Some(Token::LParen),
      ')' => Some(Token::RParen),
      ',' => Some(Token::Comma),
      '+' => Some(Token::Plus),
      '-' => Some(Token::Minus),
      '*' => Some(Token::Asterisk),
      '/' => Some(Token::Slash),
      '>' => Some(Token::GT),
      '<' => Some(Token::LT),
      '{' => Some(Token::LBrace),
      '}' => Some(Token::RBrace),
      '=' => {  // TODO: consider handling two-char tokens more generally.
        if let Some('=') = self.chars.peek() {
          self.chars.next();
          Some(Token::Eq)
        } else {
          Some(Token::Assign)
        }
      },
      '!' => {
        if let Some('=') = self.chars.peek() {
          self.chars.next();
          Some(Token::NotEq)
        } else {
          Some(Token::Illegal)
        }
      },
      c_ => {
        if is_monkey_letter(&c_) {  // read identifier
          let ident_str = self.accumulate_while(is_monkey_letter, c_);
          parse_keyword(&ident_str).or(Some(Token::Ident(ident_str)))
        } else if is_monkey_digit(&c_){
          let num_str = self.accumulate_while(is_monkey_digit, c_);
          let num = num_str.parse::<u32>().expect("not a number");
          Some(Token::Int(num))
        } else {
          Some(Token::Illegal)
        }
      }
    }
  }
}

fn is_monkey_letter(c: &char) -> bool {
  c.is_ascii_alphabetic() || *c == '_'
}

fn is_monkey_digit(c: &char) -> bool {
  c.is_ascii_digit()
}

fn is_monkey_whitespace(c: &char) -> bool {
  c.is_whitespace()
}

fn parse_keyword(s: &str) -> Option<Token> {
  match s {
    "let" => Some(Token::Let),
    "fn" => Some(Token::Function),
    "true" => Some(Token::True),
    "false" => Some(Token::False),
    "if" => Some(Token::If),
    "else" => Some(Token::Else),
    "return" => Some(Token::Return),
    _ => None,
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_test() {
    println!("I'm a test of the tests");
  }

  #[test]
  fn test_lexer_single_chars() {
    let mut l = Lexer::new("{}+=");
    assert_eq!(l.next(), Some(Token::LBrace));
    assert_eq!(l.next(), Some(Token::RBrace));
    assert_eq!(l.next(), Some(Token::Plus));
    assert_eq!(l.next(), Some(Token::Assign));
    assert_eq!(l.next(), None);
  }

    #[test]
  fn test_lexer_ident() {
    let mut l = Lexer::new("{}+=asd_f=");
    assert_eq!(l.next(), Some(Token::LBrace));
    assert_eq!(l.next(), Some(Token::RBrace));
    assert_eq!(l.next(), Some(Token::Plus));
    assert_eq!(l.next(), Some(Token::Assign));
    assert_eq!(l.next(), Some(Token::Ident(String::from("asd_f"))));
    assert_eq!(l.next(), Some(Token::Assign));
    assert_eq!(l.next(), None);

    let mut l = Lexer::new("{}+=asd_f");
    assert_eq!(l.next(), Some(Token::LBrace));
    assert_eq!(l.next(), Some(Token::RBrace));
    assert_eq!(l.next(), Some(Token::Plus));
    assert_eq!(l.next(), Some(Token::Assign));
    assert_eq!(l.next(), Some(Token::Ident(String::from("asd_f"))));
    assert_eq!(l.next(), None);
  }

  #[test]
  fn test_eat_whitespace() {
    let mut l = Lexer::new("  {}   +=asd_f  =  ");
    assert_eq!(l.next(), Some(Token::LBrace));
    assert_eq!(l.next(), Some(Token::RBrace));
    assert_eq!(l.next(), Some(Token::Plus));
    assert_eq!(l.next(), Some(Token::Assign));
    assert_eq!(l.next(), Some(Token::Ident(String::from("asd_f"))));
    assert_eq!(l.next(), Some(Token::Assign));
    assert_eq!(l.next(), None);
  }

  #[test]
  fn test_some_code() {
    let mut l = Lexer::new("
      let stuff = fn(x, y) {
        return x + y + 3;
      };
    ");
    assert_eq!(l.next(), Some(Token::Let));
    assert_eq!(l.next(), Some(Token::Ident(String::from("stuff"))));
    assert_eq!(l.next(), Some(Token::Assign));
    assert_eq!(l.next(), Some(Token::Function));
    assert_eq!(l.next(), Some(Token::LParen));
    assert_eq!(l.next(), Some(Token::Ident(String::from("x"))));
    assert_eq!(l.next(), Some(Token::Comma));
    assert_eq!(l.next(), Some(Token::Ident(String::from("y"))));
    assert_eq!(l.next(), Some(Token::RParen));
    assert_eq!(l.next(), Some(Token::LBrace));
    assert_eq!(l.next(), Some(Token::Return));
    assert_eq!(l.next(), Some(Token::Ident(String::from("x"))));
    assert_eq!(l.next(), Some(Token::Plus));
    assert_eq!(l.next(), Some(Token::Ident(String::from("y"))));
    assert_eq!(l.next(), Some(Token::Plus));
    assert_eq!(l.next(), Some(Token::Int(3)));
    assert_eq!(l.next(), Some(Token::Semicolon));
    assert_eq!(l.next(), Some(Token::RBrace));
    assert_eq!(l.next(), Some(Token::Semicolon));
    assert_eq!(l.next(), None);
  }

  // TODO: ought to test some more things, but we'll call this good for now.
}