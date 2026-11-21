#[cfg(test)]
mod test {
    use crate::{Lexer, TokenKind};

    #[test]
    fn advances_through_characters() {
        let mut lexer = Lexer::new("12+3");

        assert_eq!(lexer.advance(), Some('1'));
        assert_eq!(lexer.advance(), Some('2'));
        assert_eq!(lexer.advance(), Some('+'));
        assert_eq!(lexer.peek(), Some('3'));
    }

    #[test]
    fn recognizes_numbers() {
        let mut lexer = Lexer::new("123.45");

        assert_eq!(lexer.read_number(), "123.45");
    }

    #[test]
    fn recognizes_identifiers() {
        let mut lexer = Lexer::new("nth_root");

        assert_eq!(lexer.read_identifier(), "nth_root");
    }

    #[test]
    fn tokenizes_expression() {
        let mut lexer = Lexer::new("2 + sqrt(16) * nth_root(27, 3)");

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens
                .iter()
                .map(|token| token.kind.clone())
                .collect::<Vec<_>>(),
            vec![
                TokenKind::Number("2".into()),
                TokenKind::Plus,
                TokenKind::Identifier("sqrt".into()),
                TokenKind::LeftParen,
                TokenKind::Number("16".into()),
                TokenKind::RightParen,
                TokenKind::Multiply,
                TokenKind::Identifier("nth_root".into()),
                TokenKind::LeftParen,
                TokenKind::Number("27".into()),
                TokenKind::Comma,
                TokenKind::Number("3".into()),
                TokenKind::RightParen,
                TokenKind::EOF,
            ]
        );
    }
}
