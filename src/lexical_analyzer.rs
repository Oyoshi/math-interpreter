use crate::token::Token;

pub struct Lexer {
    input: String,
    current_pos: i32,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &String) -> Lexer {
        let mut lexer = Lexer {
            input: input.to_string(),
            current_pos: -1,
            current_char: None,
        };
        lexer.advance();

        lexer
    }

    pub fn get_next_token(&mut self) -> Token {
        while self.current_char != None {
            if self.current_char.unwrap().is_whitespace() {
                self.advance();
                continue;
            }

            if self.current_char.unwrap().is_digit(10) {
                return Token::INTEGER(self.integer());
            }

            match self.current_char.unwrap() {
                '+' => {
                    self.advance();
                    return Token::PLUS;
                }
                '-' => {
                    self.advance();
                    return Token::MINUS;
                }
                '*' => {
                    self.advance();
                    return Token::MUL;
                }
                '/' => {
                    self.advance();
                    return Token::DIV;
                }
                '^' => {
                    self.advance();
                    return Token::POW;
                }
                '(' => {
                    self.advance();
                    return Token::LPAREN;
                }
                ')' => {
                    self.advance();
                    return Token::RPAREN;
                }
                _ => panic!("Invalid character"),
            }
        }

        Token::EOF
    }

    fn advance(&mut self) {
        self.current_pos += 1;
        if self.current_pos > self.input.len() as i32 - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input.as_bytes()[self.current_pos as usize] as char);
        }
    }

    fn integer(&mut self) -> i32 {
        let begin_idx = self.current_pos.clone() as usize;
        while let Some(current_char) = self.current_char {
            if current_char.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        let end_idx = self.current_pos.clone() as usize;
        let result = &self.input[begin_idx..end_idx];

        result.parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexical_analyzer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_empty_token() {
        let text = String::from("");
        let mut lexer = Lexer::new(&text);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_integer_token() {
        let text = String::from("2137");
        let mut lexer = Lexer::new(&text);
        assert_eq!(lexer.get_next_token(), Token::INTEGER(2137));
    }

    #[test]
    fn test_non_integer_token() {
        let text = String::from("+");
        let mut lexer = Lexer::new(&text);
        assert_eq!(lexer.get_next_token(), Token::PLUS);
    }

    #[test]
    #[should_panic]
    fn test_token_does_not_exist() {
        let text = String::from("&");
        let mut lexer = Lexer::new(&text);
        lexer.get_next_token();
    }
}
