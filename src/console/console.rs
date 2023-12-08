use crate::calculator::Calculator;

use num_traits::identities::*;
use std::io::{self, BufRead, StdinLock, StdoutLock, Write};

#[derive(Debug)]
pub struct Console {
    stdin: StdinLock<'static>,
    stdout: StdoutLock<'static>,
    calculator: Calculator,
}

impl Console {
    const CLEAR_SCREEN: &[u8] = b"\x1b[2J\x1b[1;1H";

    const WELCOME_MESSAGE: &[u8] =
        b"\nCalculator V 0.3.1 the code is now much cleaner!\n\nUse 'help' for list of commands\n";
    const NEW_EXPRESSION_MESSAGE: &[u8] = b"\nEnter an expression or a command: ";
    const ANSWER_MASSAGE: &[u8] = b"Answer: ";
    const ERROR_MESSAGE: &[u8] = b"Error: ";
    const HELP_MASSAGE: &[u8] = b"
    help - display this massage\n
    end - close calculator\n
    reset - reset your last answer back to 0\n
    clear - clear screen\n
    a - represents your last answer\n
    p - represents PI number\n
    e - represents EXPONENT number\n";

    const HELP_COMMAND: &str = "help";
    const PROGRAM_END_COMMAND: &str = "end";
    const ANSWER_RESET_COMMAND: &str = "reset";
    const CLEAR_COMMAND: &str = "clear";

    pub fn new() -> Self {
        Self {
            stdin: io::stdin().lock(),
            stdout: io::stdout().lock(),
            calculator: Calculator::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.stdout.write_all(Self::WELCOME_MESSAGE)?;
        self.stdout.flush()?;

        loop {
            self.stdout.write_all(Self::NEW_EXPRESSION_MESSAGE)?;
            self.stdout.flush()?;

            let buffer = String::from_utf8_lossy(self.stdin.fill_buf()?)
                .trim_end_matches('\n')
                .to_lowercase();

            match buffer.as_str() {
                Self::HELP_COMMAND => self.stdout.write_all(Self::HELP_MASSAGE)?,
                Self::PROGRAM_END_COMMAND => break,
                Self::ANSWER_RESET_COMMAND => self.calculator.reset(),
                Self::CLEAR_COMMAND => self.stdout.write_all(Self::CLEAR_SCREEN)?,
                _ => match self.calculator.calculate(&buffer) {
                    Ok(answer) => self.print_calculator_output(Self::ANSWER_MASSAGE, answer)?,
                    Err(calculator_error) => {
                        self.print_calculator_output(Self::ERROR_MESSAGE, calculator_error)?
                    }
                },
            }
            self.stdout.flush()?;

            self.stdin.consume(buffer.len() + usize::one());
        }

        Ok(())
    }

    fn print_calculator_output(
        &mut self,
        message_type: &[u8],
        message: impl ToString,
    ) -> io::Result<()> {
        let output = String::from_utf8_lossy(message_type).to_string()
            + &message.to_string()
            + &'\n'.to_string();

        self.stdout.write_all(output.as_bytes())?;

        Ok(())
    }
}
