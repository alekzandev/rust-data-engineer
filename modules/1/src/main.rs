use module1::add;
use module1::div;
use module1::mul;
use module1::sub;

fn main() {
    println!("Hello, world!");
    println!("Add: {}", add(10, 5));
    println!("Sub: {}", sub(10, 5));
    println!("Mul: {}", mul(10, 5));
    println!("Div: {}", div(10, 5));
}
