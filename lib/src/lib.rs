use std::fs;

/// Handle input from the user and convert to String.
pub fn handle_input_file(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

#[cfg(test)]
mod tests {
    use crate::handle_input_file;

    #[test]
    #[should_panic(expected = "Something went wrong reading the file")]
    fn bad_path() {
        assert_eq!("", handle_input_file("input/test"));
    }

    #[test]
    fn incorrect_file() {
        assert_ne!(
            String::from("Je suis un test avec des caractères spéciaux ^^ 🥳"),
            handle_input_file("input/test.txt")
        );
    }

    #[test]
    fn correct_file() {
        assert_eq!(
            String::from("Je suis un test avec des\ncaractères spéciaux ^^\n🥳"),
            handle_input_file("input/test.txt")
        );
    }
}
