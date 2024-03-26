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

fn mutable_reference() {
    let mut s = String::from("Hello");
    let r1 = &mut s;
    r1.insert_str(5, ", World!");
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
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

    mutable_reference();
}

pub fn change_value(change: bool) {
    let mut s = "abc".to_string();
    if change {
        s = "def".to_string();
    }
    print!("{s}")
}

pub fn change_value_and_use(change: bool) {
    let mut s: String = "".to_string();
    if change {
        s = "def".to_string();
        println!("Changed!")
    }
    println!("String: {s}");
}

pub fn mut_value(mut mut_val: i32) {
    println!("Original num: {mut_val}");
    mut_val = 3;
    println!("Changed num: {mut_val}");
}

pub fn mut_ref(mut_ref: &mut i32) {
    println!("{mut_ref}");
    *mut_ref = 3;
    println!("{mut_ref}");
}
