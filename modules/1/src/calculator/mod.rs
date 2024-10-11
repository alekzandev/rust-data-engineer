// Add function
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Sub function
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

// Mul function
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

// Div function
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide by zero error");
    } else {
        a / b
    }
}
