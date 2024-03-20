use crate::structs::Base;

pub trait Hello {
    fn hello(&self) -> String;
}

impl Hello for Base {
    fn hello(&self) -> String {
        format!("Hello, {}!", self.get_hidden_thing())
    }
}

impl Hello for String {
    fn hello(&self) -> String {
        format!("Hello, {}!", self)
    }
}
