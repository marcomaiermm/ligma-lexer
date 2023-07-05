use std::collections::HashMap;

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    DOT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

#[allow(dead_code)]
pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: char,
    input: String,
    keywords: HashMap<String, Token>,
}

#[allow(dead_code)]
impl Lexer {
    #[allow(dead_code)]
    pub fn new(input: &str) -> Lexer {
        let mut keywords = HashMap::new();
        keywords.insert(String::from("let"), Token::LET);

        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: '0',
            input: String::from(input),
            keywords: keywords,
        };
        lex.read_char();
        return lex;
    }

    #[allow(dead_code)]
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '.' => Token::DOT,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            // '=' => Token::LET,
            // '=' => Token::FUNCTION,
            // '=' => Token::IDENT,
            // '=' => Token::INT,
            '0' => Token::EOF,
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    //TODO: Check if identifier is keyword
                    return Token::IDENT(String::from(ident));
                } else {
                    return Token::ILLEGAL;
                }
            }
        };
        self.read_char();
        return token;
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        return &self.input[position..self.position];
    }
}

#[allow(dead_code)]
fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z';
}

#[cfg(test)]
mod test {
    use crate::{Lexer, Token};

    #[test]
    fn get_next_token() {
        let input = "=+-.(){},;penis";
        // let monkey_input = "
        //     let five = 5;
        //     let ten = 10;
        //
        //     let add = fn(x, y) {
        //       x + y;
        //     };
        //
        //     let result = add(five, ten);
        // ";
        let tests = [
            (Token::ASSIGN, '='),
            (Token::PLUS, '+'),
            (Token::MINUS, '-'),
            (Token::DOT, '.'),
            (Token::LPAREN, '('),
            (Token::RPAREN, ')'),
            (Token::LBRACE, '{'),
            (Token::RBRACE, '}'),
            (Token::COMMA, ','),
            (Token::SEMICOLON, ';'),
            (Token::IDENT(String::from("penis")), 'x'),
            (Token::EOF, '0'),
        ];
        let mut lexer_luther = Lexer::new(input);
        for test in tests {
            let token = lexer_luther.next_token();
            println!("Expected {:?}, received {:?}", test.0, token);
            assert_eq!(test.0, token);
        }
    }
}
