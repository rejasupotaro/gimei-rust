use Samplable;

#[derive(Debug)]
pub struct Address {
    pub prefecture: Prefecture,
    pub city: City,
    pub town: Town,
}

impl Address {
    pub fn new() -> Address {
        Address {
            prefecture: Prefecture::new(),
            city: City::new(),
            town: Town::new(),
        }
    }

    pub fn kanji(&self) -> String {
        let prefecture = self.prefecture.kanji();
        let city = self.city.kanji();
        let town = self.town.kanji();
        format!("{}{}{}", prefecture, city, town)
    }

    pub fn hiragana(&self) -> String {
        let prefecture = self.prefecture.hiragana();
        let city = self.city.hiragana();
        let town = self.town.hiragana();
        format!("{}{}{}", prefecture, city, town)
    }

    pub fn katakana(&self) -> String {
        let prefecture = self.prefecture.katakana();
        let city = self.city.katakana();
        let town = self.town.katakana();
        format!("{}{}{}", prefecture, city, town)
    }
}

#[derive(Debug)]
pub struct Prefecture {
    prefectures: Vec<String>,
}

impl Prefecture {
    pub fn new() -> Prefecture {
        let prefectures = super::ADDRESSES.get("prefecture")
                                          .and_then(|n| n.sample().as_slice())
                                          .unwrap()
                                          .iter()
                                          .map(|n| n.as_str().unwrap().to_string())
                                          .collect::<Vec<String>>();
        Prefecture { prefectures: prefectures }
    }

    pub fn kanji(&self) -> String {
        self.prefectures.get(0).unwrap().to_string()
    }

    pub fn hiragana(&self) -> String {
        self.prefectures.get(1).unwrap().to_string()
    }

    pub fn katakana(&self) -> String {
        self.prefectures.get(2).unwrap().to_string()
    }
}


#[derive(Debug)]
pub struct City {
    cities: Vec<String>,
}

impl City {
    pub fn new() -> City {
        let cities = super::ADDRESSES.get("city")
                                     .and_then(|n| n.sample().as_slice())
                                     .unwrap()
                                     .iter()
                                     .map(|n| n.as_str().unwrap().to_string())
                                     .collect::<Vec<String>>();
        City { cities: cities }
    }

    pub fn kanji(&self) -> String {
        self.cities.get(0).unwrap().to_string()
    }

    pub fn hiragana(&self) -> String {
        self.cities.get(1).unwrap().to_string()
    }

    pub fn katakana(&self) -> String {
        self.cities.get(2).unwrap().to_string()
    }
}

#[derive(Debug)]
pub struct Town {
    towns: Vec<String>,
}

impl Town {
    pub fn new() -> Town {
        let towns = super::addresses()
                        .get("town")
                        .and_then(|n| n.sample().as_slice())
                        .unwrap()
                        .iter()
                        .map(|n| n.as_str().unwrap().to_string())
                        .collect::<Vec<String>>();
        Town { towns: towns }
    }

    pub fn kanji(&self) -> String {
        self.towns.get(0).unwrap().to_string()
    }

    pub fn hiragana(&self) -> String {
        self.towns.get(1).unwrap().to_string()
    }

    pub fn katakana(&self) -> String {
        self.towns.get(2).unwrap().to_string()
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
        let prefecture = Prefecture::new();
        assert!(!prefecture.kanji().is_empty());
        assert!(!prefecture.hiragana().is_empty());
        assert!(!prefecture.katakana().is_empty());
    }

    #[test]
    fn city() {
        let city = City::new();
        assert!(!city.kanji().is_empty());
        assert!(!city.hiragana().is_empty());
        assert!(!city.katakana().is_empty());
    }

    #[test]
    fn town() {
        let town = Town::new();
        assert!(!town.kanji().is_empty());
        assert!(!town.hiragana().is_empty());
        assert!(!town.katakana().is_empty());
    }
}
