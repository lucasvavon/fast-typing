#[derive(Debug, PartialEq)]
pub enum InputResult {
    Success,
    Error,
    Nothing,
}

pub struct Game {
    pub sample_text: String,
    pub input_text: String,
}

impl Game {
    pub fn input(&mut self, input_char: char) -> InputResult {
        if input_char == '\u{8}' {
            if self.input_text.is_empty() {
                return InputResult::Nothing;
            }
            self.input_text.pop();
        } else {
            self.input_text.push(input_char);
        }
        let expected_char = self.sample_text.chars().nth(self.input_text.len() - 1).unwrap();

        if input_char != expected_char {
            return InputResult::Error;
        }


        InputResult::Success
    }
}