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
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
