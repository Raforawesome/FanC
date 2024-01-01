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

    #[regex(r#"slog\("[^\)]*"\)+"#, slog)]
    Slog(String),

    #[regex(r"\([^)]*\)", paren)]
    Paren(String),

    #[regex(r"[0-9]\w*", |lex| lex.slice().parse::<i32>().unwrap())]
    IntLiteral(i32),

    #[regex(r"([0-9]+\.[0-9]+)\w*", |lex| lex.slice().parse::<f32>().unwrap())]
    FloatLiteral(f32),

	#[regex(r#"(".*")+"#, |lex| lex.slice().to_string())]
	StringLiteral(String),

    #[regex(r"[a-zA-Z]\w+", |lex| lex.slice().to_string())]
    Ident(String),

	#[token(";")]
	Semi,

	#[regex("{[^}]*}+", block)]
	Block(Vec<Token>),
}

// Handlers
fn block(lex: &mut Lexer<Token>) -> Vec<Token> {
    todo!()
}

fn slog(lex: &mut Lexer<Token>) -> String {
    let s: &str = lex.slice();
    s[6..s.len() - 1].to_string()
}

fn paren(lex: &mut Lexer<Token>) -> Option<String> {
    let s: &str = lex.slice();
    Some(s[1..s.len() - 1].to_string())
}

// Handler to get inner string of slog function.
// TODO: Reimplement function to not use an O(n) loop.
// Bounds checking both ends is a much better (and faster) solution.
// Currently disabled and rewrite has been inlined.
// #[cfg(disabled)]
// fn slog(lex: &mut Lexer<Token>) -> Option<String> {
//     let slice = lex.slice();
//     let mut left: usize = 0;
//     let mut right: usize = slice.len() - 1;
//     let mut db: bool = false;
//     let (mut db1, mut db2) = (false, false);
//     for (i, c) in slice.chars().enumerate() {
//         if c == '"' {
//             if !db {
//                 left = i + 1;
//             } else {
//                 db = true;
//                 right = i;
//             }
//             if db1 == false {
//                 db1 = true;
//             } else {
//                 db2 = true;
//             }
//         }
//     }
//     db2.then(|| db1.then(|| &slice[left..right]).unwrap().to_string())
// }

