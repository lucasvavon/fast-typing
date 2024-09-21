#[derive(Debug, PartialEq)]
pub enum InputResult {
    Success,
    Error,
    Nothing,
}

pub enum Action {
    Insert(char),
    Remove,
    Jump,
}

pub struct Game {
    pub sample_text: String,
    pub input_text: String,
}

impl Game {
    pub fn new(sample_text: String) -> Game {
        Game {
            sample_text,
            input_text: String::new(),
        }
    }

    pub fn handle_action(&mut self, action: Action) -> InputResult {
        match action {
            Action::Insert(input_char) => {
                self.input_text.push(input_char);
                InputResult::Success
            }
            Action::Remove => {
                if self.input_text.is_empty() {
                    return InputResult::Nothing;
                }
                self.input_text.pop();
                InputResult::Success
            }
            Action::Jump => {
                self.jump_to_next_word();
                InputResult::Success
            }
        }
    }

    pub fn input(&mut self, input_char: char) -> InputResult {
        if input_char == '\u{8}' {
            return self.handle_action(Action::Remove);
        }

        // Get the expected character
        let expected_char = if let Some(char) = self.sample_text.chars().nth(self.input_text.len()) {
            char
        } else {
            return InputResult::Nothing; // No more characters expected
        };


        // Check for space and handle jump
        if expected_char != ' ' && input_char == ' ' {
            self.handle_action(Action::Jump);
        }

        // Verify the input character matches the expected character
        if input_char != expected_char {
            return InputResult::Error; // Or include a message with more details
        } else {
            self.handle_action(Action::Insert(input_char));
        }

        InputResult::Success
    }

    /*pub fn get_next_word_relative_position(&mut self) -> usize {
        let current_position: usize = self.input_text.len() - 1;
        let part = self.sample_text[current_position..].to_string();
        let next_word_pos = part.find(' ').unwrap();
        println!("{}", part);
        next_word_pos
    }*/


    pub fn jump_to_next_word(&mut self) -> usize {
        let current_position: usize = self.input_text.len() - 1;
        let part = self.sample_text[current_position..].to_string();
        let next_word_pos = part.find(' ').unwrap();
        for _ in 0..next_word_pos {
            self.input_text.push(' ');
        }
        next_word_pos
    }
}