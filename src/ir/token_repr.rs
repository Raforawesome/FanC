use std::collections::HashMap;

/// Type to represent all basic types of C.
/// This enum isn't meant to be exhaustive, rather
/// only cover the define-able types in C that make up other C
/// concepts, such as structs or functions.
#[derive(Debug, Clone, Copy, PartialEq)]
enum CType {
    Int,
    Short,
    Float,
    Double,
    Long,
    UInt,
    UShort,
    ULong,
    ULLong,
    Char,
    Void,
    Ptr(CType),
    Array(CType),
}

#[derive(Debug, Clone, PartialEq)]
struct CStructField(CType, String);

#[derive(Debug, Clone, PartialEq)]
struct CStruct {
    name: String,
    field: Vec<CStructField>,
}
