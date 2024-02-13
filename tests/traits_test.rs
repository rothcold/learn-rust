#[cfg(test)]
mod traits_test {
    use learn_rust::structs::Base;
    use learn_rust::traits::Hello;

    #[test]
    fn base_traits_test() {
        let base = Base::new("No matter".to_string(), "World".to_string());
        let hello = base.hello();
        assert!(hello.eq("Hello, World!"));
    }
}
