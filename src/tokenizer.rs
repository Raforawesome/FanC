use logos::{Lexer, Logos};

/// A list of C tokens that are used in the parser.
#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("int")]
    Int,

    #[token("char")]
    Char,

    #[token("long")]
    Long,

    #[token("short")]
    Short,

    #[token("float")]
    Float,

    #[token("double")]
    Double,

    #[token("void")]
    Void,

    #[token("struct")]
    Struct,

    #[token("union")]
    Union,

    #[token("enum")]
    Enum,

    #[token("signed")]
    Signed,

    #[token("unsigned")]
    Unsigned,

    #[token("const")]
    Const,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("do")]
    Do,

    #[token("while")]
    While,

    #[token("if")]
    If,

    #[token("return")]
    Return,

    #[regex(r"[0-9]\w*", |lex| lex.slice().parse::<i32>().unwrap())]
    IntLiteral(i32),

    #[regex(r"([0-9]+\.[0-9]+)\w*", |lex| lex.slice().parse::<f32>().unwrap())]
    FloatLiteral(f32),

    #[regex(r#"("[^"]*")+"#, string)]
    StringLiteral(String),

    #[regex(r"[a-zA-Z]\w+", |lex| lex.slice().to_string())]
    Ident(String),

    #[token(";")]
    Semi,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("{")]
    BlockOpen,

    #[token("}")]
    BlockClose,
}

// Handlers
fn string(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    let len = slice.len();
    slice[1..(len - 1)].to_owned()
}
