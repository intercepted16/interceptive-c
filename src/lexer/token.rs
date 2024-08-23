#[derive(Debug, PartialEq, Clone)]
pub(crate) enum VariableValue {
    Integer(i64),
    String(String),
    Undefined,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    EOF,
    Print,
    PrintWithArgument(String, Vec<String>),
    Variable(String, VariableValue),
}
