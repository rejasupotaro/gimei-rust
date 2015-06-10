#[derive(Debug)]
pub struct Name {
    pub first: First,
    pub last: Last,
    pub gender: Gender
}

impl Name {
    pub fn new() -> Name {
        let first = First {
            hiragana: "はるな".to_string()
        };

        let last = Last {
            hiragana: "さいとう".to_string()
        };

        Name {
            first: first,
            last: last,
            gender: Gender::Female
        }
    }

    pub fn hiragana(&self) -> String {
        let first = self.first.hiragana.to_string();
        let last = self.last.hiragana.to_string();
        vec![last, first].connect(" ")
    }

    pub fn is_female(&self) -> bool {
        self.gender.is_female()
    }

    pub fn is_male(&self) -> bool {
        self.gender.is_male()
    }
}

#[derive(Debug)]
pub enum Gender {
    Female,
    Male
}

impl Gender {
    pub fn is_female(&self) -> bool {
        match *self {
            Gender::Female => true,
            _ => false
        }
    }

    pub fn is_male(&self) -> bool {
        match *self {
            Gender::Male => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct First {
    pub hiragana: String
}

#[derive(Debug)]
pub struct Last {
    pub hiragana: String
}
