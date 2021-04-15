#[cfg(test)]
mod test_lexer {
    use crate::lexer::{Lexer, Op, Token};

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new(String::from("2x"));
        assert_eq!(
            lexer.tokens(),
            vec![Token::Int(2), Token::Var(String::from("x"))]
        );
    }

    #[test]
    fn test_ops() {
        let mut lexer = Lexer::new(String::from("+-*/"));
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::Op(Op::Plus),
                Token::Op(Op::Minus),
                Token::Op(Op::Multiply),
                Token::Op(Op::Divide),
            ]
        )
    }

    #[test]
    fn test_parens() {
        let mut lexer = Lexer::new(String::from("(1x)"));
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::LParen,
                Token::Int(1),
                Token::Var(String::from("x")),
                Token::RParen,
            ]
        )
    }

    #[test]
    fn test_linear_eq() {
        let mut lexer = Lexer::new(String::from("10x = 3 * (x + 3)"));
        assert_eq!(
            lexer.tokens(),
            vec![
                // ERROR: current impl records 10 as `[Token::Int(1), Token::Int(0)]`
                Token::Int(10),
                Token::Var(String::from("x")),
                Token::Eq,
                Token::Int(3),
                Token::Op(Op::Multiply),
                Token::LParen,
                Token::Var(String::from("x")),
                Token::Op(Op::Plus),
                Token::Int(3),
                Token::RParen,
            ]
        )
    }

    #[test]
    fn test_combine_like_terms() {
        let mut lexer = Lexer::new(String::from("8x + y / 3 * (2y - x)"));
        assert_eq!(
            lexer.tokens(),
            vec![
                Token::Int(8),
                Token::Var(String::from("x")),
                Token::Op(Op::Plus),
                Token::Var(String::from("y")),
                Token::Op(Op::Divide),
                Token::Int(3),
                Token::Op(Op::Multiply),
                Token::LParen,
                Token::Int(2),
                Token::Var(String::from("y")),
                Token::Op(Op::Minus),
                Token::Var(String::from("x")),
                Token::RParen,
            ]
        )
    }
}
