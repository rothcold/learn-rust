#[cfg(test)]
mod memory_test {
    #[test]
    fn test_memory_allocate() {
        for i in 1..1073741824 {
            mk1();
            println!("{i}");
        }
    }
    fn mk1() {
        mk2()
    }
    fn mk2() {
        let _: [u8; 1048576] = [0; 1048576];
    }
}
