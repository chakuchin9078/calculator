use crate::calculator_error::CalculatorError;
use crate::calculator_operator::CalculatorOperator;

use num_traits::identities::*;
use std::collections::{HashMap, VecDeque};
use std::f64::consts::{E, PI};

#[derive(Debug)]
pub struct Calculator {
    previous_answer: f64,
    operation_priority: HashMap<char, u8>,
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
            previous_answer: zero(),
            operation_priority: Self::OPERATION_PRIORITY.into(),
        }
    }

    pub fn calculate(&mut self, infix_expression: &str) -> Result<f64, CalculatorError> {
        let (postfix_expression, mut local_numbers) =
            (self.to_postfix(infix_expression)?, Vec::new());

        println!("{postfix_expression:?}");

        postfix_expression.iter().try_for_each(|cell| {
            let result = match CalculatorOperator::try_from(
                cell.chars()
                    .nth(cell.len() - 1)
                    .expect("cell shouldn't be empty"),
            ) {
                Ok(operator) => {
                    let second_number =
                        local_numbers
                            .pop()
                            .ok_or(CalculatorError::OperatorWithoutANumber(
                                operator.clone().into(),
                            ))?;

                    let first_number = match operator {
                        CalculatorOperator::Inverse => None,
                        _ => Some(local_numbers.pop().ok_or(
                            CalculatorError::OperatorWithoutANumber(operator.clone().into()),
                        )?),
                    };

                    operator.execute(first_number, second_number)
                }
                Err(_) => Ok(cell.parse().expect("cell should be a number")),
            }?;

            local_numbers.push(result);

            Ok(())
        })?;

        let &current_answer = local_numbers.last().ok_or(CalculatorError::EmptyInput)?;

        self.previous_answer = current_answer;
        Ok(current_answer)
    }

    pub fn reset(&mut self) {
        self.previous_answer.set_zero();
    }

    fn to_postfix(&self, infix_expression: &str) -> Result<Vec<Box<str>>, CalculatorError> {
        let (mut postfix_expression, mut operations_stack) = (Vec::new(), VecDeque::new());

        let mut i = zero();
        while i < infix_expression.len() {
            let character = infix_expression
                .chars()
                .nth(i)
                .expect("i should be less that infix expression's length");

            let (is_digit, is_first_iteration) = (character.is_ascii_digit(), i.is_zero());

            if !is_digit && is_first_iteration {
                postfix_expression.push(self.previous_answer.to_string().into());
            }

            if is_digit {
                let string_number = self.get_string_number(infix_expression, i)?;

                i += string_number.len() - usize::one();
                postfix_expression.push(string_number);
            } else if character == '.' {
                return Err(CalculatorError::DotWithoutANumber);
            } else if character == 'a' {
                postfix_expression.push(self.previous_answer.to_string().into());
            } else if character == 'p' {
                postfix_expression.push(PI.to_string().into());
            } else if character == 'e' {
                postfix_expression.push(E.to_string().into());
            } else if character == '(' {
                operations_stack.push_front(character);
            } else if character == ')' {
                let position = operations_stack
                    .iter()
                    .position(|&operation| operation == '(')
                    .ok_or(CalculatorError::ClosingBracketWithoutAPair)?;

                let operations = operations_stack
                    .drain(..position)
                    .map(|operation| operation.to_string().into());

                postfix_expression.extend(operations);

                operations_stack
                    .pop_front()
                    .expect("operations stack shouldn't be empty");
            } else if self.operation_priority.contains_key(&character) {
                let mut current_operation = character;

                let previous_character_is_operation = (!is_first_iteration)
                    && self.operation_priority.contains_key(
                        &infix_expression.chars().nth(i - usize::one()).expect(
                            "i should be less that infix expression's length and more than 0",
                        ),
                    );

                if current_operation == '-' && previous_character_is_operation {
                    current_operation = '~';
                }

                let current_operation_priority = self
                    .operation_priority
                    .get(&current_operation)
                    .expect("operation priority should contain current operation key");

                let position = operations_stack
                    .iter()
                    .position(|operation| {
                        self.operation_priority
                            .get(operation)
                            .expect("operation priority should contain operation key")
                            < current_operation_priority
                    })
                    .unwrap_or(operations_stack.len());

                let operations = operations_stack
                    .drain(..position)
                    .map(|operation| operation.to_string().into());

                postfix_expression.extend(operations);

                operations_stack.push_front(current_operation);
            } else {
                return Err(CalculatorError::UnknownSymbol(character));
            }

            i += usize::one();
        }

        postfix_expression.extend(
            operations_stack
                .into_iter()
                .map(|operation| {
                    if operation == '(' {
                        Err(CalculatorError::OpeningBracketWithoutAPair)
                    } else {
                        Ok(operation.to_string().into())
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        );

        Ok(postfix_expression)
    }

    fn get_string_number(
        &self,
        string: &str,
        position: usize,
    ) -> Result<Box<str>, CalculatorError> {
        let mut has_dot = false;

        Ok(string
            .chars()
            .skip(position)
            .map_while(|character| {
                if character.is_ascii_digit() {
                    Some(Ok(character))
                } else if character == '.' {
                    if has_dot {
                        Some(Err(CalculatorError::DotWithoutANumber))
                    } else {
                        has_dot = true;
                        Some(Ok(character))
                    }
                } else {
                    None
                }
            })
            .collect::<Result<String, _>>()?
            .into())
    }
}
