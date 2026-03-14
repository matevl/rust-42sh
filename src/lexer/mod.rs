use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(String),
    If,
    Then,
    Else,
    Elif,
    Fi,
    Semi,       // ;
    NewLine,    // \n
    Less,       // <
    Great,      // >
    DGreat,     // >>
    LessAnd,    // <&
    GreatAnd,   // >&
    LessGreat,  // <>
    DLess,      // <<
    CLobber,    // >|
    Pipe,       // |
    And,        // &
    IoNumber(String), // 2>
    EOF,
}

/// Lexer struct that reads from any character iterator.
pub struct Lexer<I: Iterator<Item = char>> {
    input: Peekable<I>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new(input: I) -> Self {
        Lexer {
            input: input.peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() && c != '\n' {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self, first: char) -> String {
        let mut word = String::new();
        word.push(first);
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() || "&|;<>(){}".contains(c) {
                break;
            }
            word.push(c);
            self.input.next();
        }
        word
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.next() {
            Some(c) => match c {
                '\n' => Token::NewLine,
                ';' => Token::Semi,
                '|' => Token::Pipe, 
                '&' => Token::And, 
                '<' => {
                    if let Some(&'<') = self.input.peek() {
                        self.input.next();
                        Token::DLess
                    } else if let Some(&'&') = self.input.peek() {
                        self.input.next();
                        Token::LessAnd
                    } else if let Some(&'>') = self.input.peek() {
                        self.input.next();
                        Token::LessGreat
                    } else {
                        Token::Less
                    }
                }
                '>' => {
                    if let Some(&'>') = self.input.peek() {
                        self.input.next();
                        Token::DGreat
                    } else if let Some(&'&') = self.input.peek() {
                        self.input.next();
                        Token::GreatAnd
                    } else if let Some(&'|') = self.input.peek() {
                        self.input.next();
                        Token::CLobber
                    } else {
                        Token::Great
                    }
                }
                '#' => {
                    // Comment: skip until newline
                    while let Some(&next_c) = self.input.peek() {
                        if next_c == '\n' {
                            break;
                        }
                        self.input.next();
                    }
                    self.next_token()
                }
                _ => {
                    let word = self.read_word(c);
                    match word.as_str() {
                        "if" => Token::If,
                        "then" => Token::Then,
                        "else" => Token::Else,
                        "elif" => Token::Elif,
                        "fi" => Token::Fi,
                        _ => {
                            if word.chars().all(|c| c.is_digit(10)) {
                                if let Some(&next_c) = self.input.peek() {
                                    if next_c == '<' || next_c == '>' {
                                        Token::IoNumber(word)
                                    } else {
                                        Token::Word(word)
                                    }
                                } else {
                                    Token::Word(word)
                                }
                            } else {
                                Token::Word(word)
                            }
                        },
                    }
                }
            },
            None => Token::EOF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let input = "ls -l; echo hello";
        let mut lexer = Lexer::new(input.chars());
        assert_eq!(lexer.next_token(), Token::Word("ls".to_string()));
        assert_eq!(lexer.next_token(), Token::Word("-l".to_string()));
        assert_eq!(lexer.next_token(), Token::Semi);
        assert_eq!(lexer.next_token(), Token::Word("echo".to_string()));
        assert_eq!(lexer.next_token(), Token::Word("hello".to_string()));
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}
