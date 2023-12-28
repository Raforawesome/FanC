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

--- 

## Unique Behaviours
Because why not have some fun with it?

Some things that are different from other compilers:
### The `slog` function
The `slog` (**s**ystem **log**) function is a special function that prints to the console. 
It does not need to be imported, and can't print anything except a single string.
It's main use is really just for debugging, and for most cases you should use `printf` instead.

Example:
```c
int main() {
    log("Hello, world!");
    return 0;
}
```
