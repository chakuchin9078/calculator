use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, PartialEq, Eq)]
pub enum CalculatorError {
    EmptyInput,
    UnknownSymbol(char),
    OperatorWithoutANumber(char),
    ClosingBracketWithoutAPair,
    OpeningBracketWithoutAPair,
    DotWithoutANumber,
    DivisionByZero,
}

impl Display for CalculatorError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use CalculatorError::*;

        match self {
            EmptyInput => formatter.write_str("empty input"),
            UnknownSymbol(char) => formatter
                .write_str(("unknown symbol '".to_owned() + &char.to_string() + "'").as_str()),
            OperatorWithoutANumber(character) => formatter.write_str(
                ("use of '".to_owned() + &(character).to_string() + "' operation without a number")
                    .as_str(),
            ),
            ClosingBracketWithoutAPair => {
                formatter.write_str("use of a closing bracket without a pair")
            }
            OpeningBracketWithoutAPair => {
                formatter.write_str("use of an opening bracket without a pair")
            }
            DotWithoutANumber => formatter.write_str("use of a dot without a number before it"),
            DivisionByZero => formatter.write_str("division by zero"),
        }
    }
}

impl Error for CalculatorError {}
