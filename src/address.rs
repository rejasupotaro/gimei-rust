use japanese::Japanese;
use samplable::Samplable;

#[derive(Debug)]
pub struct Address {
    pub prefecture: Japanese,
    pub city: Japanese,
    pub town: Japanese,
}

impl Address {
    pub fn new() -> Address {
        Address {
            prefecture: Japanese::from_array(super::ADDRESSES.get("prefecture")
                                                             .and_then(|n| n.sample().as_slice())
                                                             .unwrap()
                                                             .iter()
                                                             .map(|n| n.as_str().unwrap())
                                                             .collect::<Vec<&str>>()),
            city: Japanese::from_array(super::ADDRESSES.get("city")
                                                       .and_then(|n| n.sample().as_slice())
                                                       .unwrap()
                                                       .iter()
                                                       .map(|n| n.as_str().unwrap())
                                                       .collect::<Vec<&str>>()),
            town: Japanese::from_array(super::addresses()
                                           .get("town")
                                           .and_then(|n| n.sample().as_slice())
                                           .unwrap()
                                           .iter()
                                           .map(|n| n.as_str().unwrap())
                                           .collect::<Vec<&str>>()),
        }
    }

    pub fn kanji(&self) -> String {
        format!("{}{}{}",
                self.prefecture.kanji,
                self.city.kanji,
                self.town.kanji)
    }

    pub fn hiragana(&self) -> String {
        format!("{}{}{}",
                self.prefecture.hiragana,
                self.city.hiragana,
                self.town.hiragana)
    }

    pub fn katakana(&self) -> String {
        format!("{}{}{}",
                self.prefecture.katakana,
                self.city.katakana,
                self.town.katakana)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kanji() {
        let address = Address::new();
        assert!(!address.kanji().is_empty());
    }

    #[test]
    fn hiragana() {
        let address = Address::new();
        assert!(!address.hiragana().is_empty());
    }

    #[test]
    fn katakana() {
        let address = Address::new();
        assert!(!address.katakana().is_empty());
    }

    #[test]
    fn prefecture() {
        let prefecture = Address::new().prefecture;
        assert!(!prefecture.kanji.is_empty());
        assert!(!prefecture.hiragana.is_empty());
        assert!(!prefecture.katakana.is_empty());
    }

    #[test]
    fn city() {
        let city = Address::new().city;
        assert!(!city.kanji.is_empty());
        assert!(!city.hiragana.is_empty());
        assert!(!city.katakana.is_empty());
    }

    #[test]
    fn town() {
        let town = Address::new().town;
        assert!(!town.kanji.is_empty());
        assert!(!town.hiragana.is_empty());
        assert!(!town.katakana.is_empty());
    }
}
