pub struct Base {
    pub some_thing: String,
    hidden_thing: String,
}

impl Base {
    pub fn new(some_thing: String, hidden_thing: String) -> Base {
        Base {
            some_thing,
            hidden_thing,
        }
    }
    pub fn get_hidden_thing(&self) -> String {
        self.hidden_thing.clone()
    }
}
