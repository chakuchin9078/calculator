#[derive(Debug)]
pub struct CalculatorCell {
    number: Option<f64>,
    operator: Option<char>
}

impl CalculatorCell {
    pub fn new_number(number: f64) -> Self {
        Self { 
            number: Some(number),
            operator: None 
        }
    }

    pub fn new_operator(operator: char) -> Self {
        Self { 
            number: None,
            operator: Some(operator) 
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        self.number
    }

    pub fn is_number(&self) -> bool {
        self.number.is_some()
    }

    pub fn get_operator(&self) -> Option<char> {
        self.operator
    }

    pub fn _is_operator(&self) -> bool {
        self.operator.is_some()
    }
}