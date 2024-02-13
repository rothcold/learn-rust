#[cfg(test)]
mod struts_test {
    use learn_rust::structs::Base;
    #[test]
    fn base_struct_test() {
        let base = Base::new("some thing".to_string(), "No matter".to_string());
        assert!(base.some_thing.eq("some thing"))
    }
}
