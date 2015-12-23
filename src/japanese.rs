#[derive(Debug)]
pub struct Japanese {
    pub kanji: String,
    pub hiragana: String,
    pub katakana: String,
}

impl Japanese {
    pub fn new(kanji: &str, hiragana: &str, katakana: &str) -> Japanese {
        Japanese {
            kanji: kanji.to_string(),
            hiragana: hiragana.to_string(),
            katakana: katakana.to_string(),
        }
    }

    pub fn from_array(strings: Vec<&str>) -> Japanese {
        Japanese::new(strings.get(0).unwrap(),
                      strings.get(1).unwrap(),
                      strings.get(2).unwrap())
    }
}
