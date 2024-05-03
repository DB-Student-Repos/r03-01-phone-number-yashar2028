pub fn number(user_number: &str) -> Option<String> {
        let mut only_digits = user_number
        .as_bytes()
        .iter()
        .cloned()
        .filter(|b| b.is_ascii_digit())
        .collect::<Vec<_>>();

        if only_digits.len() == 11 && only_digits[0] == b'1' {
            only_digits.remove(0);
        }

        if only_digits.len() == 10 {
            match only_digits[0..10] {
            [b'2'..=b'9', _, _, b'2'..=b'9', _, _, _, _, _, _] => Some(String::from_utf8(only_digits).unwrap()),
            _ => None,
            }
        }

        else {
            None
        }
}
