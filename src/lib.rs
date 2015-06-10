pub mod name;

use std::collections::HashMap;

pub fn names() -> HashMap<String, Vec<String>> {
    let mut names = HashMap::new();
    names.insert("first_name".to_string(), vec!["A".to_string(), "B".to_string()]);
    names
}

pub fn name() -> name::Name {
    name::Name::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_full_name_as_hiragana() {
        let gimei = name();
        assert_eq!("さいとう はるな".to_string(), gimei.hiragana());
    }

    #[test]
    fn get_first_name_as_hiragana() {
        let gimei = name();
        assert_eq!("はるな".to_string(), gimei.first.hiragana);
    }

    #[test]
    fn get_last_name_as_hiragana() {
        let gimei = name();
        assert_eq!("さいとう".to_string(), gimei.last.hiragana);
    }

    #[test]
    fn is_female() {
        let gimei = name();
        assert!(gimei.is_female());
    }

    #[test]
    fn is_male() {
        let gimei = name();
        assert!(!gimei.is_male());
    }
}
