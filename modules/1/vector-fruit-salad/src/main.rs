/*
This program create a fruit salad by scrambling a list of fruits.
A vector is a growable array. It can grow or shrink in size and is
one of the most useful data structures in Rust.
A vector is represented by the Vec<T> type.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruit: Vec<&str> = vec![
        "Orange",
        "Fig",
        "Pineapple",
        "Banana",
        "Kiwi",
        "Apple",
    ];

    // Scramble the fruits
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // Print the salad
    println!("Fruit salad:");
    for (i, f) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", f);
        } else {
            println!("and {}", f);
        }
    }
}
