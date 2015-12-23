use toml::Value;
use rand;

pub trait Samplable {
    fn sample(&self) -> &Value;
}

impl Samplable for Value {
    fn sample(&self) -> &Value {
        match *self {
            Value::Array(..) => {
                let vec = &self.as_slice().unwrap();
                let index = rand::random::<usize>() % vec.len();
                &vec[index]
            }
            _ => self,
        }
    }
}
