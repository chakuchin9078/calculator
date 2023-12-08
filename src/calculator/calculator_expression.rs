use crate::calculator_error::CalculatorError;

use num_traits::{identities::*, Pow};
use std::ops::Div;

#[derive(Debug)]
pub enum CalculatorExpression {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl CalculatorExpression {
    pub fn execute(&self, first_number: f64, second_number: f64) -> Result<f64, CalculatorError> {
        use CalculatorExpression::*;

        match self {
            Add => Ok(first_number + second_number),
            Subtract => Ok(first_number - second_number),
            Multiply => Ok(first_number * second_number),
            Divide => Ok(first_number.checked_div(second_number)?),
            Power => Ok(first_number.pow(second_number)),
        }
    }
}

impl From<char> for CalculatorExpression {
    fn from(value: char) -> Self {
        use CalculatorExpression::*;

        match value {
            '+' => Add,
            '-' => Subtract,
            '*' => Multiply,
            '/' => Divide,
            '^' => Power,
            _ => panic!("Invalid expression '{value}'"),
        }
    }
}

trait CheckedDiv<T: Div> {
    fn checked_div(&self, other: T) -> Result<T, CalculatorError>;
}

impl CheckedDiv<f64> for f64 {
    fn checked_div(&self, other: f64) -> Result<f64, CalculatorError> {
        if other.is_zero() {
            Err(CalculatorError::DivisonByZero)
        } else {
            Ok(self / other)
        }
    }
}
