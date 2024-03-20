#[cfg(test)]
mod traits_test {
    use learn_rust_examples::structs::Base;
    use learn_rust_examples::traits::Hello;

    #[test]
    fn base_traits_test() {
        let base = Base::new("No matter".to_string(), "World".to_string());
        let hello = base.hello();
        assert!(hello.eq("Hello, World!"));
        let s = "Rothcold".to_string();
        println!("{}", s.hello());
    }
}
