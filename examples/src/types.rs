use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub struct MyError {
    details: String,
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}
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

pub fn option(x: Option<i32>) {
    if let Some(y) = x {
        println!("x is {y}");
        println!("x is {}", x.unwrap())
    } else {
        println!("x is None")
    }
}

pub fn result(x: Result<String, Box<dyn Error>>) {
    if let Ok(y) = x {
        println!("x is {y}");
    } else {
        println!("Error is {}", x.unwrap_err().to_string())
    }
}
