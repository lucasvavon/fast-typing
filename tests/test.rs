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
    fn check_space_on_word_insert_spaces_to_next_word() {
        let mut game = Game { sample_text: String::from("hello world"), input_text: String::from("h") };
        game.jump_to_next_word();
        assert_eq!(game.input_text, String::from("h     "))
    }

    #[test]
    fn check_space_on_word_jump_to_next_word() {
        let mut game = Game { sample_text: String::from("hello world"), input_text: String::from("h") };
        let result = game.input(' ');
        assert_eq!(result, InputResult::Jump)
    }

    #[test]
    fn check_count_error() {
        let mut game = Game { sample_text: String::from("hello world"), input_text: String::from("helli wolfd") };
        let errors = game.count_errors();
        assert_eq!(errors, 3)
    }

    #[test]
    fn check_backspace_to_previous_position_on_bad_space() {
        let mut game = Game { sample_text: String::from("hello world qiu str"), input_text: String::from("hello w     ") };
        let jump_size = game.back_to_previous_position();
        let mut assert = assert_eq!(game.input_text, String::from("hello w"));
        assert = assert_eq!(jump_size, 5);

        assert
    }
}
