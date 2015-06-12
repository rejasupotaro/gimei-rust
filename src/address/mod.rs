use Samplable;

#[derive(Debug)]
pub struct Address {
    pub prefecture: Prefecture,
    pub city: City
}

impl Address {
    pub fn new() -> Address {
        Address {
            prefecture: Prefecture::new(),
            city: City::new()
        }
    }

    pub fn kanji(&self) -> String {
        let prefecture = self.prefecture.kanji();
        let city = self.city.kanji();
        format!("{}{}", prefecture, city)
    }

    pub fn hiragana(&self) -> String {
        let prefecture = self.prefecture.hiragana();
        let city = self.city.hiragana();
        format!("{}{}", prefecture, city)
    }

    pub fn katakana(&self) -> String {
        let prefecture = self.prefecture.katakana();
        let city = self.city.katakana();
        format!("{}{}", prefecture, city)
    }
}

#[derive(Debug)]
pub struct Prefecture {
    prefectures: Vec<String>
}

impl Prefecture {
    pub fn new() -> Prefecture {
        let prefectures = super::addresses()
            .get("prefecture")
            .and_then(|n| n.sample().as_slice())
            .unwrap()
            .iter()
            .map(|n| n.as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        Prefecture {
            prefectures: prefectures
        }
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
    cities: Vec<String>
}

impl City {
    pub fn new() -> City {
        let cities = super::addresses()
            .get("city")
            .and_then(|n| n.sample().as_slice())
            .unwrap()
            .iter()
            .map(|n| n.as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        City {
            cities: cities
        }
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
}
