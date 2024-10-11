use module1::calculator::add;
use module1::calculator::div;
use module1::calculator::mul;
use module1::calculator::sub;

fn main() {
    println!("Hello, world!");
    println!("Add: {}", add(10, 5));
    println!("Sub: {}", sub(10, 5));
    println!("Mul: {}", mul(10, 5));
    println!("Div: {}", div(10, 5));
}
