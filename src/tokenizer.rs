use logos::Logos;

/// A list of C tokens that are used in the parser.
#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
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
	// The following keywords are compiler
	// suggestions that don't really mean anything
	// in the context of the parser. Therefore they are
	// ignored.
	// #[token("volatile")]
	// Volatile,
	// #[token("auto")]
	// Auto,
	// #[token("register")]
	// Register,
}
