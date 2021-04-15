#[cfg(test)]
pub mod test;

// use std::convert::TryFrom;
// use std::str::FromStr;

/// Tokens for algebraic expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Any number, float or int. E.g. 4, 2.3, 9, 3.1529, -2
    Int(isize),
    /// A variable used in the equation or expression. E.g. x, y, p, o, n, m
    Var(String),
    /// An equals sign: `=`.
    Eq,
    /// An operator: +, -, /, *, ^.
    Op(Op),
    /// (
    LParen,
    /// )
    RParen,
}

/// An operator: +, -, / *.
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    /// Operator: +
    Plus,
    /// Operator: -
    Minus,
    /// Operator: *
    Multiply,
    /// Operator: /
    Divide,
    /// Operator: ^
    Exponent,
}

/// Handles and holds the tokens for the given input.
#[derive(Debug)]
pub struct Lexer {
    /// Vector of tokens in the given input.
    pub tokens: Vec<Token>,
    /// The expression or equation the user gives.
    input: String,
    /// The current position of the Lexer.
    current_pos: usize,
    start_pos: usize,
}

impl Lexer {
    /// Gives the tokens of the given input.
    pub fn tokens(input: String) -> Vec<Token> {
        let mut lexer = Self {
            tokens: Vec::new(),
            input,
            current_pos: 0,
            start_pos: 0,
        };

        while !lexer.is_end() {
            lexer.start_pos = lexer.current_pos;
            lexer.make_tokens();
        }

        lexer.tokens.clone()
    }

    // fn next_char(&mut self) -> char {
    //     self.input.chars().nth(self.current_pos).unwrap_or('\0')
    // }

    fn advance(&mut self) -> char {
        self.current_pos += 1;
        self.input.chars().nth(self.current_pos - 1).unwrap_or('\0')
    }

    fn is_end(&mut self) -> bool {
        self.current_pos >= self.input.len()
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn make_tokens(&mut self) {
        match self.advance() {
            ' ' | '\t' | '\r' => (),
            '=' => self.add_token(Token::Eq),
            '(' => self.add_token(Token::LParen),
            ')' => self.add_token(Token::RParen),
            '+' => self.add_token(Token::Op(Op::Plus)),
            '-' => self.add_token(Token::Op(Op::Minus)),
            '*' => self.add_token(Token::Op(Op::Multiply)),
            '/' => self.add_token(Token::Op(Op::Divide)),
            '^' => self.add_token(Token::Op(Op::Exponent)),
            character => {
                if character.is_digit(10) {
                    // HACK: turn `char` into `isize`
                    self.add_token(Token::Int((character as u32 - '0' as u32) as isize));
                } else {
                    self.add_token(Token::Var(String::from(character)));
                }
            }
        }
    }

    // fn make_number(&mut self) {
    //     let mut dot_count = 0;

    //     while self.next_char().is_ascii_alphanumeric() && !self.is_end() {
    //         self.advance();
    //     }

    //     if self.next_char() == '.' {
    //         dot_count += 1;
    //         self.advance();
    //     }

    //     while self.next_char().is_ascii_alphanumeric() && !self.is_end() {
    //         self.advance();
    //     }

    //     let input = self.input.clone();

    //     let final_num = input[self.start_pos..self.current_pos - 1].to_owned();
    //     let parsed = isize::from_str(&final_num).unwrap();

    //     self.add_token(Token::Int(parsed));
    // }
}
