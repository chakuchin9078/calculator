use crate::calculator_error::CalculatorError;

use std::collections::HashMap;
use std::f64::consts::{E, PI};

#[derive(Debug)]
pub struct Calculator {
    previous_answer: f64,
    opration_priority: HashMap<char, u8>,
}

impl Calculator {
    const OPERATION_PRIORITY: [(char, u8); 7] = [
        ('(', 0),
        ('+', 1),
        ('-', 1),
        ('*', 2),
        ('/', 2),
        ('^', 3),
        ('~', 4),
    ];

    pub fn new() -> Self {
        Self {
            previous_answer: 0.0,
            opration_priority: HashMap::from(Self::OPERATION_PRIORITY),
        }
    }

    pub fn calculate(&mut self, infix_expression: &str) -> Result<f64, CalculatorError> {
        let postfix_expression = self.to_postfix(infix_expression)?;

        if postfix_expression.is_empty() {
            return Err(CalculatorError::EmptyInput);
        }

        let mut local_numbers = Vec::new();

        let mut i = 0;
        while i < postfix_expression.len() {
            let cell = postfix_expression.get(i).unwrap();

            if cell.starts_with(|character: char| character.is_ascii_digit()) {
                local_numbers.push(cell.parse().unwrap());
            } else if self
                .opration_priority
                .contains_key(&cell.chars().nth(0).unwrap())
            {
                let operator = cell.chars().nth(0).unwrap();

                if operator == '~' {
                    if let Some(number) = local_numbers.pop() {
                        local_numbers.push(self.math(0.0, number, '-')?);
                    }
                } else if let (Some(second_number), Some(first_number)) =
                    (local_numbers.pop(), local_numbers.pop())
                {
                    local_numbers.push(self.math(first_number, second_number, operator)?);
                } else {
                    return Err(CalculatorError::OperationWihtoutANumber(operator));
                }
            }

            i += 1;
        }

        match local_numbers.pop() {
            Some(number) => {
                self.previous_answer = number;
                Ok(number)
            }
            None => Err(CalculatorError::Uknown),
        }
    }

    pub fn reset(&mut self) {
        self.previous_answer = 0.0;
    }

    fn to_postfix(&self, infix_expression: &str) -> Result<Vec<Box<str>>, CalculatorError> {
        let (mut postfix_expression, mut operations_stack) = (Vec::new(), Vec::new());

        let mut i = 0;
        while i < infix_expression.len() {
            let character = infix_expression.chars().nth(i).unwrap();
            let is_digit = character.is_ascii_digit();

            if !is_digit && i == 0 {
                postfix_expression.push(self.previous_answer.to_string().into_boxed_str());
            }

            if is_digit {
                match self.get_string_number(infix_expression, i) {
                    Ok((string_number, position)) => {
                        postfix_expression.push(string_number);
                        i = position
                    }
                    Err(calculator_error) => return Err(calculator_error),
                }
            } else if character == '.' {
                return Err(CalculatorError::DotWithoutANumber);
            } else if character == 'a' {
                postfix_expression.push(self.previous_answer.to_string().into_boxed_str());
            } else if character == 'p' {
                postfix_expression.push(PI.to_string().into_boxed_str())
            } else if character == 'e' {
                postfix_expression.push(E.to_string().into_boxed_str())
            } else if character == '(' {
                operations_stack.push(character);
            } else if character == ')' {
                let postfix_expression_length = postfix_expression.len();

                postfix_expression.append(
                    operations_stack
                        .iter()
                        .rev()
                        .map_while(|operation| {
                            if *operation != '(' {
                                Some(operation.to_string().into_boxed_str())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .as_mut(),
                );

                operations_stack.drain(
                    operations_stack.len()
                        - (postfix_expression.len() - postfix_expression_length)..,
                );

                if operations_stack.is_empty() || *operations_stack.last().unwrap() != '(' {
                    return Err(CalculatorError::ClosingBracketWithoutAPair);
                }

                operations_stack.pop();
            } else if self.opration_priority.contains_key(&character) {
                let mut current_operator = character;

                if current_operator == '-'
                    && (i == 0
                        || self
                            .opration_priority
                            .contains_key(&infix_expression.chars().nth(i - 1).unwrap()))
                {
                    current_operator = '~';
                }

                let (postfix_expression_length, current_operator_priority) = (
                    postfix_expression.len(),
                    self.opration_priority.get(&current_operator).unwrap(),
                );

                postfix_expression.append(
                    operations_stack
                        .iter()
                        .rev()
                        .map_while(|operation| {
                            if self.opration_priority.get(operation).unwrap()
                                >= current_operator_priority
                            {
                                Some(operation.to_string().into_boxed_str())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .as_mut(),
                );

                operations_stack.drain(
                    operations_stack.len()
                        - (postfix_expression.len() - postfix_expression_length)..,
                );
                operations_stack.push(current_operator);
            } else {
                return Err(CalculatorError::UknownSymbol(character));
            }

            i += 1;
        }

        postfix_expression.append(
            operations_stack
                .iter()
                .rev()
                .map_while(|operation| {
                    if *operation != '(' {
                        Some(Ok(operation.to_string().into_boxed_str()))
                    } else {
                        Some(Err(CalculatorError::OpeningBracketWithoutAPair))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .as_mut(),
        );

        Ok(postfix_expression)
    }

    fn get_string_number(
        &self,
        string: &str,
        position: usize,
    ) -> Result<(Box<str>, usize), CalculatorError> {
        let mut has_dot = false;

        match string
            .get(position..string.len())
            .unwrap()
            .chars()
            .map_while(|character| {
                if character.is_ascii_digit() {
                    Some(Ok(character))
                } else if character == '.' {
                    if !has_dot {
                        has_dot = true;
                        Some(Ok(character))
                    } else {
                        Some(Err(CalculatorError::DotWithoutANumber))
                    }
                } else {
                    None
                }
            })
            .collect::<Result<String, _>>()
        {
            Ok(string_number) => Ok((
                string_number.clone().into_boxed_str(),
                position + string_number.len() - 1,
            )),
            Err(calculator_error) => Err(calculator_error),
        }
    }

    fn math(
        &self,
        first_number: f64,
        second_number: f64,
        expression: char,
    ) -> Result<f64, CalculatorError> {
        match expression {
            '+' => Ok(first_number + second_number),
            '-' => Ok(first_number - second_number),
            '*' => Ok(first_number * second_number),
            '/' => match second_number as i64 {
                0 => Err(CalculatorError::DivisonByZero),
                _ => Ok(first_number / second_number),
            },
            '^' => Ok(f64::powf(first_number, second_number)),
            _ => Err(CalculatorError::UknownOperator(expression)),
        }
    }
}
