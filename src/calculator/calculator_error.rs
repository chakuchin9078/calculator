use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, PartialEq, Eq)]
pub enum CalculatorError {
    Uknown,
    EmptyInput,
    UknownSymbol(char),
    OperationWihtoutANumber(char),
    ClosingBracketWithoutAPair,
    OpeningBracketWithoutAPair,
    DotWithoutANumber,
    DivisonByZero,
}

impl Display for CalculatorError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use CalculatorError::*;

        match self {
            Uknown => formatter.write_str("uknown error"),
            EmptyInput => formatter.write_str("empty input"),
            UknownSymbol(char) => {
                formatter.write_str(("uknown symbol".to_owned() + &char.to_string()).as_str())
            }
            OperationWihtoutANumber(char) => formatter.write_str(
                ("use of".to_owned() + &char.to_string() + "operation wihtout a number").as_str(),
            ),
            ClosingBracketWithoutAPair => {
                formatter.write_str("use of a closing bracket without a pair")
            }
            OpeningBracketWithoutAPair => {
                formatter.write_str("use of an opening bracket without a pair")
            }
            DotWithoutANumber => formatter.write_str("use of a dot without a number before it"),
            DivisonByZero => formatter.write_str("divison by zero"),
        }
    }
}

impl Error for CalculatorError {}
