use logos::Logos;

// Define the tokens for the lexer
#[derive(Logos, Debug, PartialEq)]
enum Token {
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
    Identifier,

    // Strings
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    // Comments
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    BlockComment,

    // Numbers (e.g., 143)
    #[regex(r"[0-9]+")]
    Number,

    // Whitespace (ignored)
    #[regex(r"[ \t\n\f]+", logos::skip)]

    // Error handling
    #[error]
    Error,
}
