extern crate rand;

#[derive(Debug)]
pub enum Gender {
    Female,
    Male
}

impl Gender {
    pub fn sample() -> Gender {
        match rand::random::<usize>() % 2 {
            0 => Gender::Female,
            _ => Gender::Male
        }
    }

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

    pub fn type_str(&self) -> &'static str {
        match *self {
            Gender::Female => "female",
            Gender::Male => "male"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_female() {
        assert!(Gender::Female.is_female());
        assert!(!Gender::Male.is_female());
    }

    #[test]
    fn is_male() {
        assert!(!Gender::Female.is_male());
        assert!(Gender::Male.is_male());
    }

    #[test]
    fn type_str() {
        assert_eq!("female", Gender::Female.type_str());
        assert_eq!("male", Gender::Male.type_str());
    }
}
