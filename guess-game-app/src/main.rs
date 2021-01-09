use std::io;
extern crate rand;//导包


use rand::random;
fn get_guess() -> u8{
    loop {
        println!("Input guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("could not read from stdin");
        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("could not understand input {}", e)
        }
    }
}

fn handle_guess(guess:u8, correct:u8) -> bool{
    if guess>correct{
        println!("Too high");
        false
    }else if guess<correct {
        println!("Too low");
        false
    }else {
        println!("You got it");
        true
    }
}

fn main() {
    println!("Welcome to no guessing game");

    let correct:u8 = random();
    println!("correct value is: {}", correct);
    
    loop {
        let guess = get_guess();
        if handle_guess(guess, correct){
            break;
        }
    }
    
}
