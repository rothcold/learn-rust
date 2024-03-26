#[cfg(test)]
mod types_test {

    use std::error::Error;

    use learn_rust_examples::types;

    #[test]
    fn basic_types_test() {
        types::basic_types();
    }

    #[test]
    fn result_test() {
        types::result(Ok("Some sentence".to_string()));
        types::result(Err(Box::new(types::MyError::new("Error is coming"))))
    }
}
