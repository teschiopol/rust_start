use std::io; // I/O library come from standard library

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // add mut because by default variables are immutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // Manage 'Err' result case

    println!("You guessed: {guess}");
    /*
    * println!("x = {} and y = {}", x, y);
    * Another solution for placeholder print
    */
}

// cargo run
// cargo test
