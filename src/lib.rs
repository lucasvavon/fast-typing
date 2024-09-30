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
            // Gestion du caractère backspace
            return self.handle_action(Action::Remove);
        }

        // Obtenir le caractère attendu
        let expected_char = if let Some(c) = self.sample_text.chars().nth(self.input_text.len()) {
            c
        } else {
            return InputResult::Nothing; // Plus de caractères attendus
        };

        if expected_char != ' ' && input_char == ' ' {
            return self.handle_action(Action::Jump);
        }

        // Vérifier si le caractère saisi correspond au caractère attendu
        if input_char != expected_char {
            // Insérer quand même le caractère incorrect pour maintenir la longueur
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
        // On récupère la longueur minimale entre les deux chaînes
        let min_len = self.sample_text.len().min(self.input_text.len());

        // On initialise un compteur de différences
        let mut errors = 0;

        // On compare les caractères des deux chaînes jusqu'à la longueur minimale
        for (c1, c2) in self.sample_text.chars().zip(self.input_text.chars()) {
            if c1 != c2 {
                errors += 1;
            }
        }

        // On ajoute la différence de longueur si une chaîne est plus longue que l'autre
        errors += self.sample_text.len().max(self.input_text.len()) - min_len;

        errors
    }
}