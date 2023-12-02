use crate::calculator::Calculator;

use std::io::{self, Write, BufRead, StdinLock, StdoutLock};

#[derive(Debug)]
pub struct Console {
    stdin: StdinLock <'static>,
    stdout: StdoutLock <'static>,
    calculator: Calculator
}

impl Console {
    const PROGRAM_END_COMMAND: &str = "end";
    const ANSWER_RESET_COMMAND: &str = "reset";
    const HELP_COMMAND: &str = "help";

    const HELP_MASSAGE: &str = "
        end - end program\n
        reset - reset your last answer to 0\n
        help - get this massage\n
        a - get your last answer\n
        p - PI number\n
        e - EXPONENT number\n";

    pub fn new() -> Self {
        Self {
            stdin: io::stdin().lock(),
            stdout: io::stdout().lock(),
            calculator: Calculator::new()
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        writeln!(self.stdout, "\nCalculator V 0.3 (answer memorization, floating point numbers, constants, small code optimizations and readability improvements)\n")?;
        writeln!(self.stdout, "Use 'help' for list of commands\n")?;

        loop {
            write!(self.stdout, "Enter an expression or a command: ")?;
            self.stdout.flush()?;

            let mut buffer = String::new();
            self.stdin.read_line(&mut buffer)?;

            buffer = buffer [0..=buffer.len() - 2].to_string().to_lowercase();

            match buffer.as_str() {
                Self::PROGRAM_END_COMMAND => break,
                Self::ANSWER_RESET_COMMAND => self.calculator.reset(),
                Self::HELP_COMMAND => writeln!(self.stdout, "{}", Self::HELP_MASSAGE)?,
                _ =>
                    match self.calculator.calculate(&buffer) {
                        Ok(ok) => writeln!(self.stdout, "Answer: {ok}\n")?,
                        Err(err) => writeln!(self.stdout, "Error: {err}\n")?
                    }
            }
        }

        Ok(())
    }
}