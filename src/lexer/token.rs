use strum_macros::Display;


#[derive(Debug, PartialEq, Clone, Display)]
pub(crate) enum VariableValue {
    Integer(i64),
    String(String),
    Undefined,
}
/// Represents different types of tokens in a lexer or parser.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    /// Indicates the end of the file or input stream.
    EOF,

    /// Represents a print statement without arguments.
    Print,

    /// Represents a print statement with an argument.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print.
    /// * `arguments` - A list of additional arguments for the print statement.
    PrintWithArgument(String, Vec<String>),

    /// Represents a variable declaration or usage.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable.
    /// * `value` - The value associated with the variable.
    Variable(String, VariableValue),

    /// Represents a function definition.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `definition` - The function's definition or body.
    /// * `arguments` - A list of argument names for the function.
    FunctionDefinition(String, String, Vec<String>),

    /// Represents a function call.
    ///
    /// # Arguments
    /// * `name` - The name of the function.
    /// * `arguments` - A list of arguments to pass to the function.
    FunctionCall(String, Vec<String>),
}
