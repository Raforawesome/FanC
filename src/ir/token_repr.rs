use std::collections::HashMap;

/// Type to represent all basic types of C.
/// This enum isn't meant to be exhaustive, rather
/// only cover the define-able types in C that make up other C
/// concepts, such as structs or functions.
#[derive(Debug, Clone, PartialEq)]
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
    Ptr(Box<CType>),
    Array(Box<CType>),
}

#[derive(Debug, Clone, PartialEq)]
struct CStructField(CType, String);

#[derive(Debug, Clone, PartialEq)]
struct CStruct {
    name: String,
    field: Vec<CStructField>,
}

#[derive(Debug, Clone, PartialEq)]
struct CFunction {
    name: String,
    return_type: CType,
    params: HashMap<String, CType>,
}
