#[derive(Debug, PartialEq)]
pub enum InputResult {
    Success,
    Jump,
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
                InputResult::Jump
            }
        }
    }

    pub fn input(&mut self, input_char: char) -> InputResult {
        if input_char == '\x08' {
            // handle backspace
            return self.handle_action(Action::Remove);
        }

        // return the char or Nothing Action
        let expected_char = if let Some(c) = self.sample_text.chars().nth(self.input_text.len()) {
            c
        } else {
            return InputResult::Nothing;
        };

        if expected_char != ' ' && input_char == ' ' {
            return self.handle_action(Action::Jump);
        }

        // check if char is valid
        if input_char != expected_char {
            // insert bad char to keep the length
            self.handle_action(Action::Insert(input_char));
            return InputResult::Error;
        } else {
            self.handle_action(Action::Insert(input_char));
        }

        InputResult::Success
    }

    pub fn jump_to_next_word(&mut self) -> usize {
        let current_position: usize = self.input_text.len() - 1;
        let part = self.sample_text[current_position..].to_string();
        let next_word_pos = part.find(' ').unwrap();
        for _i in 0..next_word_pos {
            self.input_text.push(' ');
        }
        next_word_pos
    }

    pub fn count_errors(&mut self) -> usize {
        // get min length of two string
        let min_len = self.sample_text.len().min(self.input_text.len());

        // init count
        let mut errors = 0;

        // the characters in the two strings are compared up to the minimum length
        for (c1, c2) in self.sample_text.chars().zip(self.input_text.chars()) {
            if c1 != c2 {
                errors += 1;
            }
        }

        // the difference in length is added if one chain is longer than the other
        errors += self.sample_text.len().max(self.input_text.len()) - min_len;

        errors
    }
}