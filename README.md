# FanC Compiler
*ironically a very simple toy compiler.*

--- 

This project is a small minimal C compiler implemented in Rust. 
It works by tokenizing the input text, converting it into some 
basic Rust representations (ASTs), then converting those into
LLVM IR and leveraging the LLVM toolchain for codegen/optimization.

---

## Unsupported Features
Features that aren't, and probably won't be, supported:
- `volatile` keyword
- `register` keyword
- `goto` statements

## Might-be-Supported Features
These features might be supported in the future, but aren't considered essential:
- `switch` statements
- `do-while` loops
- `typedef`

## Working Features
A list of what the compiler currently supports:
- soon™️

## Planned/In-Progress Features
A list of things I'm aiming to include soon:
- Linking stdlib
- Including header files
- Other pre-processor directives
- `malloc`/`free`/`realloc`/`sizeof`
- Getting basic syntax compiling
