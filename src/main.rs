use std::io; // I/O library come from standard library
use rand::Rng;
use std::cmp::Ordering;

fn check_side_min(secret_number: u32, guess: u32) -> String {
    let diff = (100 - guess)/2;
    if (guess + diff) < secret_number{
        return "Side up!".to_string();
    }
    return "Side down".to_string();
}

fn check_side_max(secret_number: u32, guess: u32) -> String {
    let diff = guess/2;
    if (guess - diff) < secret_number{
        return "Side down!".to_string();
    }
    return "Side up".to_string();
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut count = 0;

    loop { // like a while

        println!("Please input your guess.");

        let mut guess = String::new(); // add mut because by default variables are immutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Manage 'Err' result case

        println!("You guessed: {guess}");

        let guess : u32 = match guess.trim().parse() {    // parse() convert str to number, in our case unsigned 32
            Ok(num) => num,
            Err(_) => continue,
        };     

        count += 1;            

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                let side = check_side_min(secret_number,guess);
                println!("Too small! {}", side);
            },
            Ordering::Greater => {
                let side = check_side_max(secret_number,guess);
                println!("Too big! {}", side);
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if count == 3 {
            println!("The secret number was: {secret_number}");
            println!("You lose!");
            break;
        }

        /*
        * println!("x = {} and y = {}", x, y);
        * Another solution for placeholder print
        */
    }
}

// cargo run
// cargo test
