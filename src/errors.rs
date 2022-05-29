#[derive(Debug, PartialEq, Eq)]
pub enum InterpreterError {
    StackUnderflow,
    StackOverflow,
    UnknownVariable { identifier: char },
    InvalidPointerAddress,
}
