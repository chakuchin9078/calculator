#[path = "calculator/calculator.rs"]
mod calculator;
#[path = "calculator/calculator_error.rs"]
mod calculator_error;
#[path = "calculator/calculator_expression.rs"]
mod calculator_expression;
#[path = "console/console.rs"]
mod console;
#[path = "tests/integration_test.rs"]
#[cfg(test)]
mod test;

use console::Console;
use std::io;

fn main() -> io::Result<()> {
    Console::new().run()
}
