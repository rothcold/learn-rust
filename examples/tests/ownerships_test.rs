#[cfg(test)]
mod ownerships_tests {
    use learn_rust_examples::ownerships;
    #[test]
    fn test_borrow() {
        ownerships::lend();
    }
}
