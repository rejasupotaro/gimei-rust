extern crate toml;
extern crate rustc_serialize;

pub mod name;

use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use toml::Value;
use self::name::gender::Gender;

pub fn name() -> name::Name {
    name::Name::new()
}

pub fn female() -> name::Name {
    name::Name::new_with_gender(Gender::Female)
}

pub fn male() -> name::Name {
    name::Name::new_with_gender(Gender::Male)
}

pub fn names() -> BTreeMap<String, Value> {
    load_file("data/names.toml")
}

pub fn load_file(filename: &str) -> BTreeMap<String, Value> {
    let mut input = String::new();

    File::open(&filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let mut parser = toml::Parser::new(&input);
    match parser.parse() {
        Some(toml) => toml,
        None => {
            let mut errors = vec!();
            for error in &parser.errors {
                let (loline, locol) = parser.to_linecol(error.lo);
                let (hiline, hicol) = parser.to_linecol(error.hi);
                errors.push(format!("{}:{}:{}-{}:{} error: {}",
                         filename, loline, locol, hiline, hicol, error.desc));
            }
            panic!(errors.connect("¥n"))
        }
    }
}
