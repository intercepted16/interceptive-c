use strum_macros::Display;


#[derive(Debug, PartialEq, Clone, Display)]
pub(crate) enum VariableValue {
    Integer(i64),
    String(String),
    Undefined,
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    EOF,
    Print,
    PrintWithArgument(String, Vec<String>),
    Variable(String, VariableValue),
}
