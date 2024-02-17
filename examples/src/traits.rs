use crate::structs::Base;

pub trait Hello {
    fn hello(&self) -> String;
}

impl Hello for Base {
    fn hello(&self) -> String {
        return format!("Hello, {}!", self.get_hidden_thing());
    }
}
