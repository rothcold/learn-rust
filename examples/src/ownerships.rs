fn borrow(s: String) {
    println!("{}", s);
}

fn borrow_and_giveback(s: String) -> String {
    println!("{}", s);
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn lend() {
    let s1 = String::from("Hello");
    calculate_length(&s1);
    borrow(s1);
    // println!("{}", s2); // borrow of moved value: `s`

    let mut s2 = String::from("Hello 2");
    calculate_length(&s2);
    s2 = borrow_and_giveback(s2);
    calculate_length(&s2);
    borrow(s2);
}
