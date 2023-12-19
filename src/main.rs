#[path = "calculator/calculator.rs"]
mod calculator;
#[path = "calculator/calculator_error.rs"]
mod calculator_error;
#[path = "calculator/calculator_operator.rs"]
mod calculator_operator;
#[path = "tests/calculator_test.rs"]
#[cfg(test)]
mod calculator_test;
#[path = "console/console.rs"]
mod console;

use console::Console;
use std::io;

fn main() -> io::Result<()> {
    Console::new().run()
}
