use logos::Logos;

// Define the tokens for the lexer
#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    // Keywords
    #[token("SERVER")]
    Server,
    #[token("ADDRESS")]
    Address,
    #[token("USER")]
    User,
    #[token("PORT")]
    Port,
    #[token("ENCRYPTION")]
    Encryption,
    #[token("PASSWORD")]
    Password,
    #[token("RULE")]
    Rule,
    #[token("NAME")]
    Name,
    #[token("ACTION")]
    Action,
    #[token("CONDITION")]
    Condition,
    
    // Actions
    #[token("DELETE")]
    Delete,
    #[token("MOVE")]
    Move,
    #[token("MOVETO")]
    MoveTo,

    // Conditions
    #[token("IN")]
    In,
    #[token("EQUALS")]
    Equals,

    // Delimiters
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token(";")]
    Semicolon,

    // Identifiers (e.g., your.username@domain.com, imap.something.com)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_@.]*")]
    Identifier(&'a str),

    // Strings
    #[regex(r#""([^"\\]|\\.)*""#)]
    String(&'a str),

    // Comments
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    BlockComment,

    // Numbers (e.g., 143)
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse())]
    Number(i64),

    // Whitespace (ignored)
    #[regex(r"[ \t\n\f]+", logos::skip)]

    // Error handling
    Error,
}


#[cfg(test)]
mod test_lexer {
    use crate::mrule_parser::Token;
    use logos::Logos;
    use std::fs;

    #[test]
    fn simple_correct_input() {
        let path = "./tests/mrule_parser/simple_correct_input.mrule";

        let content = match fs::read_to_string(path) {
            Ok(input) => {
                input
            }
            Err(e) => {
                panic!("Error reading file: {}", e);
            }
        };

        let tokens = Token::lexer(&content).spanned();

        let expected_output = vec![
            Token::Server,
            Token::OpenBrace,
            Token::Address,
            Token::String("test-adress.com"),
            Token::Semicolon,
            Token::User,
            Token::String("DouglasAdams"),
            Token::Semicolon,
            Token::Port,
            Token::Number(42),
            Token::Semicolon,
            Token::Encryption,
            Token::Identifier("STARTTLS"),
            Token::Semicolon,
            Token::Password,
            Token::String("password1"),
            Token::Semicolon,
            Token::CloseBrace
        ];

        for (token_span, expected) in tokens.zip(expected_output) {
            let (token, _) = token_span;
            assert_eq!(token, Ok(expected));
        }
    }
}
