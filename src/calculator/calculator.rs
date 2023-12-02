use crate::calculator_error::CalculatorError;
use crate::calculator_cell::CalculatorCell;

use std::collections::HashMap;
use std::f64::consts::{PI, E};

#[derive(Debug)]
pub struct Calculator {
    previous_answer: f64,
    opration_priority: HashMap<char, u8>
}

impl Calculator {
    const OPERATION_PRIORITY: [(char, u8); 7] = [
        ('(', 0),
        ('+', 1),
        ('-', 1),
        ('*', 2),
        ('/', 2),
        ('^', 3),
        ('~', 4)
    ];

    pub fn new() -> Self {
        Self {
            previous_answer: 0.0,
            opration_priority: HashMap::from(Self::OPERATION_PRIORITY)
        }
    }

    pub fn calculate(&mut self, infix_expression: &str) -> Result<f64, CalculatorError> {
        let infix_expression = infix_expression.to_string();
        let postfix_expression = self.to_postfix(&infix_expression)?;

        if postfix_expression.is_empty() {
            return Err(CalculatorError::EmptyInput);
        }
        
        let mut local_numbers = Vec::new();

        let mut i = 0;
        while i < postfix_expression.len() {
            let cell = &postfix_expression [i];
        
            if cell.is_number()
            {
                local_numbers.push(cell.get_number().unwrap());
            }
            else if self.opration_priority.contains_key(&cell.get_operator().unwrap())
            {
                let operator = cell.get_operator().unwrap();

                if operator == '~'
                {
                    let number = match local_numbers.pop() {
                        Some(some) => some,
                        None => return Err(CalculatorError::OperationWihtoutANumber(operator))
                    };

                    local_numbers.push(self.math(0.0, number, '-')?);
                }
                else {
                    let number_second = match local_numbers.pop() {
                        Some(some) => some,
                        None => return Err(CalculatorError::OperationWihtoutANumber(operator))
                    };

                    let number_first = match local_numbers.pop() {
                        Some(some) => some,
                        None => return Err(CalculatorError::OperationWihtoutANumber(operator))
                    };
                            
                    local_numbers.push(self.math(number_first, number_second, operator)?);
                }
            }

            i += 1;
        }

        match local_numbers.pop() {
            Some(some) => {
                self.previous_answer = some;
                Ok(some)
            },
            None => Err(CalculatorError::Uknown)
        }
    }

    pub fn reset(&mut self) {
        self.previous_answer = 0.0;
    }

    fn to_postfix(&self, infix_expression: &str) -> Result<Vec<CalculatorCell>, CalculatorError> {
        let (mut postfix_expression, mut operations_stack) = 
            (Vec::new(), Vec::new());
    
        let mut i = 0;
        while i < infix_expression.len() {
            let character = infix_expression.chars().nth(i).unwrap();
            
            if !character.is_ascii_digit() && i == 0 {
                postfix_expression.push(CalculatorCell::new_number(self.previous_answer));
            }

            if character.is_ascii_digit() {
                let number;
                (number, i) = self.get_string_number(infix_expression, i);
                postfix_expression.push(CalculatorCell::new_number(number))
            }
            else if character == '.' {
                return Err(CalculatorError::DotWithoutANumber);
            }
            else if character == 'a' {
                postfix_expression.push(CalculatorCell::new_number(self.previous_answer));
            }
            else if character == 'p' {
                postfix_expression.push(CalculatorCell::new_number(PI))
            }
            else if character == 'e' {
                postfix_expression.push(CalculatorCell::new_number(E))
            }
            else if character == '(' {
                operations_stack.push(character);
            }
            else if character == ')' {
                while !operations_stack.is_empty() && *operations_stack.last().unwrap() != '(' {
                    postfix_expression.push(CalculatorCell::new_operator(operations_stack.pop().unwrap()));
                }
    
                if operations_stack.is_empty() || *operations_stack.last().unwrap() != '(' {
                    return Err(CalculatorError::ClosingBracketWithoutAPair);
                }
    
                operations_stack.pop();
            }
            else if self.opration_priority.contains_key(&character) {
                let mut operator = character;
    
                if operator == '-' && (
                    i == 0 || (
                        i > 1 &&
                        self.opration_priority.contains_key(
                            &infix_expression
                                .chars()
                                .nth(i - 1)
                                .unwrap()
                        )
                    )
                ) {
                    operator = '~';
                }
                      
                while
                    !operations_stack.is_empty() &&
                    self.opration_priority [operations_stack.last().unwrap()] >=
                    self.opration_priority [&operator] {
                    postfix_expression.push(CalculatorCell::new_operator(operations_stack.pop().unwrap()));
                }
    
                operations_stack.push(operator);
            }
            else {
                return Err(CalculatorError::UknownSymbol(character));
            }
    
            i += 1;
        }

        for operation in operations_stack.clone() {
            if operation == '(' {
                return Err(CalculatorError::OpeningBracketWithoutAPair);
            }
    
            postfix_expression.push(CalculatorCell::new_operator(operations_stack.pop().unwrap()));
        }
    
        Ok(postfix_expression)
    }

    fn get_string_number(&self, string: &str, position: usize) -> (f64, usize) {
        let mut has_dot = false;

        let string_number =
            string
            [position..string.len()]
            .chars()
            .map_while(
                |character|
                if character.is_ascii_digit(){
                    Some(character)
                }
                else if character == '.' && !has_dot {
                    has_dot = true;
                    Some(character)
                }
                else {
                    None
                }
            )
            .collect::<String>();
    
        (string_number.parse().unwrap(), position + string_number.len() - 1)
    }
    
    fn math(&self, first_number: f64, second_number: f64, expression: char) -> Result<f64, CalculatorError> {
        match expression {
            '+' => Ok(first_number + second_number),
            '-' => Ok(first_number - second_number),
            '*' => Ok(first_number * second_number),
            '/' => 
                match second_number as i64 {
                    0 => Err(CalculatorError::DivisonByZero),
                    _ => Ok(first_number / second_number)
                },
            '^' => Ok(f64::powf(first_number, second_number)),
            _ => Err(CalculatorError::UknownOperator(expression))
        }
    }
}