#[cfg(test)]
mod control_flows_test {
    use learn_rust_examples::control_flows;

    #[test]
    fn test_loop() {
        control_flows::loop_example()
    }

    #[test]
    fn test_if_else() {
        control_flows::if_else_example(10);
        control_flows::if_else_example(1);
        control_flows::if_let_example(None);
        control_flows::if_let_example(Some(32));
    }

    #[test]
    fn test_match() {
        control_flows::match_example(None);
        control_flows::match_example(Some(32));
    }
}
