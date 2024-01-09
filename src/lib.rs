#![doc = include_str!("../README.md")]

mod ir;
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
        println!("---------------------------------------------------");
        println!("Lexing log_hello.c:");
        let content: String = std::fs::read_to_string("./input_tests/log_hello.c").unwrap();
        let mut lex = tokenizer::Token::lexer(&content);
        let mut current = lex.next();
        while let Some(res) = current {
            let slice: &str = lex.slice();
            if let Ok(t) = res {
                println!("Found token {t:?} at slice '{slice}'");
            } else {
                println!("Token error at slice '{slice}'");
            }
            current = lex.next();
        }
        println!("---------------------------------------------------");
    }
}
