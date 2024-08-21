#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    EOF,
    Print,
    PrintWithArgument(String),
    Variable(String, i64),
}