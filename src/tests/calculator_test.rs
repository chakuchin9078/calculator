use crate::calculator::Calculator;
use crate::calculator_error::CalculatorError;

#[test]
fn normal_equation() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("2+2").unwrap(), 4.0);
}

#[test]
fn floating_point_equation() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("2.2+2.2").unwrap(), 4.4);
}

#[test]
fn order_of_operations() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("2+2*2").unwrap(), 6.0);
}

#[test]
fn hard_equation() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("15/(7-(1+1))*3-(2+(1+1))*15/(7-(200+1))*3-(2+(1+1))*(15/(7-(1+1))*3-(2+(1+1))+15/(7-(1+1))*3-(2+(1+1)))").unwrap(), -30.072164948453608);
}

#[test]
fn p_and_e() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("p+e").unwrap(), 5.859874482048838);
}

#[test]
fn answer() {
    let mut calc = Calculator::new();
    calc.calculate("2+2").unwrap();
    assert_eq!(calc.calculate("a*2").unwrap(), 8.0);
}

#[test]
fn hidden_answer() {
    let mut calc = Calculator::new();
    calc.calculate("2+2").unwrap();
    assert_eq!(calc.calculate("*2").unwrap(), 8.0);
}

#[test]
fn answer_reset() {
    let mut calc = Calculator::new();
    calc.calculate("2+2").unwrap();
    calc.reset();
    assert_eq!(calc.calculate("a+2").unwrap(), 2.0);
}

#[test]
fn negative_number() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("-2-2").unwrap(), -4.0);
}

#[test]
fn harder_negative_number() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("-(-2-2)*-2").unwrap(), -8.0);
}

#[test]
fn empty_input() {
    let mut calc = Calculator::new();
    assert_eq!(calc.calculate("").unwrap_err(), CalculatorError::EmptyInput);
}

#[test]
fn unknown_symbol() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("@").unwrap_err(),
        CalculatorError::UnknownSymbol('@')
    );
}

#[test]
fn unknown_symbol_in_expression() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("2+@2").unwrap_err(),
        CalculatorError::UnknownSymbol('@')
    );
}

#[test]
fn operation_without_a_number() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("2+").unwrap_err(),
        CalculatorError::OperatorWithoutANumber('+')
    );
}

#[test]
fn closing_bracket_without_a_pair() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate(")").unwrap_err(),
        CalculatorError::ClosingBracketWithoutAPair
    );
}

#[test]
fn nested_closing_bracket_without_a_pair() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("(()))").unwrap_err(),
        CalculatorError::ClosingBracketWithoutAPair
    );
}

#[test]
fn opening_bracket_without_a_pair() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("(").unwrap_err(),
        CalculatorError::OpeningBracketWithoutAPair
    );
}

#[test]
fn nested_opening_bracket_without_a_pair() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("((())").unwrap_err(),
        CalculatorError::OpeningBracketWithoutAPair
    );
}

#[test]
fn dot_without_a_number_before() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate(".2").unwrap_err(),
        CalculatorError::DotWithoutANumber
    );
}

#[test]
fn dot_without_a_number_after() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("2.2.").unwrap_err(),
        CalculatorError::DotWithoutANumber
    );
}

#[test]
fn division_by_zero() {
    let mut calc = Calculator::new();
    assert_eq!(
        calc.calculate("2/0").unwrap_err(),
        CalculatorError::DivisionByZero
    );
}
