#![doc = include_str!("../README.md")]

/// The library file of the application.
/// This file contains all module definitions and crate-level
/// declarations. Also contains app logic for the entry point
/// of the application, located in `main.rs`.
mod tokenizer;

/// Project Unit Tests
/// A list of tests to run independently of the main program.
/// This is mainly just to ensure each step of the program
/// is working as intended during development.
#[cfg(test)]
mod tests {
    use crate::tokenizer;
    use logos::Logos;
    #[test]
    fn lex_log_hello() {
        println!("Lexing log_hello.c:");
        let content: String = std::fs::read_to_string("./input_tests/log_hello.c").unwrap();
        let mut lex: Lexer<'_, Token> = tokenizer::Token::lexer(&content);
        let mut current: Option<Result<Token, ()>> = lex.next();
        while let Some(res) = current {
            let slice: &str = lex.slice();
            if let Ok(t) = res {
                println!("Found token {t:?} at slice '{slice}'");
            } else {
                println!("Token error at slice '{slice}'");
            }
            current = lex.next();
        }
    }
}

