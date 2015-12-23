use toml::Value;
use rand::{Rng, thread_rng};

pub trait Samplable {
    fn sample(&self) -> &Value;
}

impl Samplable for Value {
    fn sample(&self) -> &Value {
        thread_rng().choose(&self.as_slice().unwrap()).unwrap()
    }
}
