#[cfg(test)]
mod show_stream_test {
    use learn_rust_macros::show_streams;

    #[show_streams]
    fn do_nothing() {}

    #[show_streams]
    fn some_body() -> String {
        "Rothcold".to_string()
    }

    #[test]
    fn macros_test() {
        do_nothing();
        some_body();
    }
}
