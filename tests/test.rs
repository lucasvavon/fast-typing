mod tests {
    use fast_typing::{Game, InputResult};

    #[test]
    fn check_correct_typed_letter() {
        let mut game = Game { sample_text: String::from("test"), input_text: String::from("") };
        let result = game.input('t');
        assert_eq!(result, InputResult::Success)
    }

    #[test]
    fn check_incorrect_typed_letter() {
        let mut game = Game { sample_text: String::from("test"), input_text: String::from("") };
        let result = game.input('x');
        assert_eq!(result, InputResult::Error)
    }

    #[test]
    fn check_backspace_remove_last_char_in_input_text() {
        let mut game = Game { sample_text: String::from("test"), input_text: String::from("tes") };
        game.input('\u{8}');
        assert_eq!(game.input_text, String::from("te"))
    }

    #[test]
    fn check_backspace_on_first_position_is_not_allowed() {
        let mut game = Game { sample_text: String::from("test"), input_text: String::from("") };
        let result = game.input('\u{8}');
        assert_eq!(result, InputResult::Nothing)
    }

    #[test]
    fn check_position_of_next_word() {
        let mut game = Game { sample_text: String::from("hello world"), input_text: String::from("h") };
        let result = game.next_word_position();
        assert_eq!(result, 6)
    }

    #[test]
    fn check_position_of_next_word_when_not_in_first_word() {
        let mut game = Game { sample_text: String::from("hello world john doe my friend"), input_text: String::from("hello world john do") };
        let result = game.next_word_position();
        assert_eq!(result, 21)
    }

    /*#[test]
    fn check_space_on_word_insert_space_to_next_word() {
        let mut game = Game { sample_text: String::from("hello world"), input_text: String::from("h") };
        //let result = game.input(' ');
        assert_eq!(game.input_text, String::from("h     "))
    }*/
}
