use logos::Logos;
use crate::mrule_lang::lexer::LexingError;

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
pub enum Lexemes {
    // Ignoring whitespace
    #[regex(r"[ \n\t\f]+", logos::skip)]
    Ignored,

    // Literals
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    UnsignedInteger(u64),
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLiteral(String),

    // Environment initiating variables
    #[token("server")]
    Server,
    #[token("rule")]
    Rule,

    // Language important signs
    #[token("(")]
    LeftBracket,
    #[token(")")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Period,
    #[token("=")]
    Assign,

    // Logical expressions
    #[token("&&")]
    #[token("and")]
    And,
    #[token("||")]
    #[token("or")]
    Or,
    #[token("!")]
    #[token("not")]
    Not,

    // Email attributes methods
    #[token("contains")]
    AttributeContains,
    #[token("is")]
    AttributeIs,
    #[token("isEmpty")]
    AttributeIsEmpty,

    // Email attributes
    #[token("subject")]
    Subject,  
    #[token("sender")]
    Sender,  
    #[token("recipient")]
    Recipient,  
    #[token("cc")]
    Cc,  
    #[token("bcc")]
    Bcc,  
    #[token("sent_date")]
    SentDate,  
    #[token("received_date")]
    ReceivedDate,  
    #[token("message_id")]
    MessageId,  
    #[token("body")]
    Body,  
    #[token("attachments")]
    Attachments,  
    #[token("is_read")]
    IsRead,  
    #[token("flags")]
    Flags,  
    #[token("priority")]
    Priority,  
    #[token("folder")]
    Folder,  
    #[token("size")]
    Size,  

    // Attachment sub-attributes  
    #[token("filename")]
    Filename,  
    #[token("content_type")]
    ContentType,  
    #[token("attachment_size")]
    AttachmentSize,  
    #[token("content")]
    Content,
}
