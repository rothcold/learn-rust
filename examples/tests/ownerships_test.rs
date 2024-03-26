#[cfg(test)]
mod ownerships_tests {
    use learn_rust_examples::ownerships;
    #[test]
    fn test_borrow() {
        ownerships::lend();
    }

    #[test]
    fn test_mut_value() {
        let num = 10;
        ownerships::mut_value(num);
        println!("Original num: {num}");
    }

    #[test]
    fn test_mut_ref() {
        let mut num = 10;
        ownerships::mut_ref(&mut num);
        println!("{num}");
    }
}
