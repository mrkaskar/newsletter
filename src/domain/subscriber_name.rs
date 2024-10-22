use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn inner(self) -> String {
        self.0
    }
    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_name_is_valid() {
        let name = "a".repeat(256);
        let result = SubscriberName::parse(name);
        assert_ok!(result);
    }
    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        let result = SubscriberName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn whitespace_only_name_is_rejected() {
        let name = " ".to_string();
        let result = SubscriberName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn empty_name_is_rejected() {
        let name = "".to_string();
        let result = SubscriberName::parse(name);
        assert_err!(result);
    }

    #[test]
    fn name_containing_invalid_characters_is_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "le guin".to_string();
        let result = SubscriberName::parse(name);
        assert_ok!(result);
    }
}
