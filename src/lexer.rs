use logos::Logos;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r#"[ \t\n\f]+"#)]
enum Token {}
