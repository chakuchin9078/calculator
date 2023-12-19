use crate::calculator_error::CalculatorError;

use num_traits::{identities::*, Pow};
use std::ops::Div;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CalculatorOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Inverse,
}

impl CalculatorOperator {
    pub fn execute(
        &self,
        first_number: Option<f64>,
        second_number: f64,
    ) -> Result<f64, CalculatorError> {
        use CalculatorOperator::*;

        match self {
            Add => Ok(first_number.expect("first_number should not be None") + second_number),
            Subtract => Ok(first_number.expect("first_number should not be None") - second_number),
            Multiply => Ok(first_number.expect("first_number should not be None") * second_number),
            Divide => Ok(first_number
                .expect("first_number should not be None")
                .checked_div(second_number))?,
            Power => Ok(first_number
                .expect("first_number should not be None")
                .pow(second_number)),
            Inverse => Ok(-second_number),
        }
    }
}

impl TryFrom<char> for CalculatorOperator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use CalculatorOperator::*;

        match value {
            '+' => Ok(Add),
            '-' => Ok(Subtract),
            '*' => Ok(Multiply),
            '/' => Ok(Divide),
            '^' => Ok(Power),
            '~' => Ok(Inverse),
            _ => Err(()),
        }
    }
}

impl From<CalculatorOperator> for char {
    fn from(value: CalculatorOperator) -> Self {
        use CalculatorOperator::*;

        match value {
            Add => '+',
            Subtract => '-',
            Multiply => '*',
            Divide => '/',
            Power => '*',
            Inverse => '~',
        }
    }
}

trait CheckedDiv<T: Div> {
    fn checked_div(&self, other: T) -> Result<T, CalculatorError>;
}

impl CheckedDiv<f64> for f64 {
    fn checked_div(&self, other: f64) -> Result<f64, CalculatorError> {
        if other.is_zero() {
            Err(CalculatorError::DivisionByZero)
        } else {
            Ok(self / other)
        }
    }
}
