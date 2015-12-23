use Samplable;
use gender::Gender;
use japanese::Japanese;

#[derive(Debug)]
pub struct Name {
    pub first: Japanese,
    pub last: Japanese,
    pub gender: Gender,
}

impl Name {
    pub fn new() -> Name {
        Name::new_with_gender(Gender::sample())
    }

    pub fn new_with_gender(gender: Gender) -> Name {
        Name {
            first: Japanese::from_array(super::NAMES.get("first_name")
                                                    .and_then(|n| n.lookup(gender.type_str()))
                                                    .and_then(|n| n.sample().as_slice())
                                                    .unwrap()
                                                    .iter()
                                                    .map(|n| n.as_str().unwrap())
                                                    .collect::<Vec<&str>>()),
            last: Japanese::from_array(super::NAMES.get("last_name")
                                                   .and_then(|n| n.sample().as_slice())
                                                   .unwrap()
                                                   .iter()
                                                   .map(|n| n.as_str().unwrap())
                                                   .collect::<Vec<&str>>()),
            gender: gender,
        }
    }

    pub fn kanji(&self) -> String {
        format!("{} {}", self.last.kanji, self.first.kanji)
    }

    pub fn hiragana(&self) -> String {
        format!("{} {}", self.last.hiragana, self.first.hiragana)
    }

    pub fn katakana(&self) -> String {
        format!("{} {}", self.last.katakana, self.first.katakana)
    }

    pub fn is_female(&self) -> bool {
        self.gender.is_female()
    }

    pub fn is_male(&self) -> bool {
        self.gender.is_male()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gender::Gender;

    #[test]
    fn kanji() {
        let name = Name::new();
        assert!(!name.kanji().is_empty());
    }

    #[test]
    fn hiragana() {
        let name = Name::new();
        assert!(!name.hiragana().is_empty());
    }

    #[test]
    fn katakana() {
        let name = Name::new();
        assert!(!name.katakana().is_empty());
    }

    #[test]
    fn gender() {
        {
            let name = Name::new_with_gender(Gender::Female);
            assert!(name.is_female());
        }
        {
            let name = Name::new_with_gender(Gender::Male);
            assert!(name.is_male());
        }
    }

    #[test]
    fn first_name() {
        let first = Name::new().first;
        assert!(!first.kanji.is_empty());
        assert!(!first.hiragana.is_empty());
        assert!(!first.katakana.is_empty());
    }

    #[test]
    fn last_name() {
        let last = Name::new().last;
        assert!(!last.kanji.is_empty());
        assert!(!last.hiragana.is_empty());
        assert!(!last.katakana.is_empty());
    }
}
