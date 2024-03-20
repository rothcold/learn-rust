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
