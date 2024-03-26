pub fn loop_example() {
    let mut x = vec![1, 2, 3];
    while let Some(y) = x.pop() {
        println!("y = {}", y);
    }

    let v = &["apples", "cake", "coffee"];

    for text in v {
        println!("I like {}.", text);
    }

    for n in 1..11 {
        println!("It's {n} now")
    }
}

pub fn if_else_example(x: i32) {
    if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is greater than 10");
    }
}

pub fn if_let_example(x: Option<i32>) {
    if let Some(y) = x {
        println!("x = {}", y);
    } else {
        println!("x is None");
    }
}

pub fn match_example(x: Option<i32>) {
    match x {
        Some(y) => println!("x is {y}"),
        None => println!("x is None"),
    }
}
