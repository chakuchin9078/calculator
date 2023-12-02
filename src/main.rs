#[path = "calculator/calculator.rs"]
mod calculator;
#[path = "calculator/calculator_error.rs"]
mod calculator_error;
#[path = "calculator/calculator_cell.rs"]
mod calculator_cell;
#[path = "console/console.rs"]
mod console;

use console::Console;
use std::io;

fn main() -> io::Result<()> {
    Console::new().run()
}