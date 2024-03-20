pub fn basic_types() {
    let b = true;
    println!("{b} is a Boolean variable.");
    let num_u8 = 255u8;
    println!("{num_u8} is a u8 variable.");
    let num_u16 = 65535u16;
    println!("{num_u16} is a u16 variable.");
    let array: [i32; 3] = [1, 2, 3];
    println!("{array:?} is a int32 array variable.");
    let slice: Box<[i32]> = Box::new([1, 2, 3]);
    println!("{slice:?} is a int32 slice variable.");
}

// Mark the function never return normally
pub fn never_return() -> ! {
    panic!("return nothing");
}
