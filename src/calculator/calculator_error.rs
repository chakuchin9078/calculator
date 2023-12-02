use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
pub enum CalculatorError {
    Uknown,
    EmptyInput,
    UknownOperator(char),
    UknownSymbol(char),
    OperationWihtoutANumber(char),
    ClosingBracketWithoutAPair,
    OpeningBracketWithoutAPair,
    DotWithoutANumber,
    DivisonByZero
}

impl Display for CalculatorError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use CalculatorError::*;

        match self {
            Uknown => write!(formatter, "uknown error"),
            EmptyInput => write!(formatter, "empty input"),
            UknownOperator(char) => write!(formatter, "uknown operator '{char}'"),
            UknownSymbol(char) => write!(formatter, "uknown symbol '{char}'"),
            OperationWihtoutANumber(char) => write!(formatter, "use of '{char}' operation wihtout a number"),
            ClosingBracketWithoutAPair => write!(formatter, "use of a closing bracket without a pair"),
            OpeningBracketWithoutAPair => write!(formatter, "use of an opening bracket without a pair"),
            DotWithoutANumber => write!(formatter, "use of a dot without a number before it"),
            DivisonByZero => write!(formatter, "divison by zero")
        }
    }
}