#[derive(Debug)]
pub enum CalculatorExpression {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
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
