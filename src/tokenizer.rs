use logos::{Lexer, Logos};

/// A list of C tokens that are used in the parser.
#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    Int,
    Char,
    Long,
    Short,
    Float,
    Double,
    Void,
    Struct,
    Union,
    Enum,
    Signed,
    Unsigned,
    Const,
    Break,
    Continue,
    Else,
    For,
    Do,
    While,
    If,
    Return,
    Slog,
    Paren(Vec<Token>),
    IntLiteral(i32),
    FloatLiteral(f32),
	StringLiteral(String),
    Ident(String),
	Semi,
	Block(Vec<Token>),
}

// Handlers
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

