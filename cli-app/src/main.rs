use std::io::{self, Write};

fn main() {
    print!("Please Enter your username: ");
    let mut username = String::new();
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut username)
        .expect("failed to read from stdin");
    let trimmed_username = username.trim();

    print!("Please Enter your password: ");
    let mut password = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read");
    println!();

    let trimmed_password = password.trim();
    if trimmed_password.eq("admin") && trimmed_username.eq("admin") {
        println!("Welcome {trimmed_username}!");
    }else{
        println!("welcome {trimmed_username}");
    }
    println!("Please choose a game from below: ");
    println!("1 - 777 Gamble");
    println!("2 - Bingo");

    println!("Enter your choice");

    let mut choice=String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read");
    let trimmed_choice = choice.trim();
    print!("{}[2J", 27 as char);
    if trimmed_choice.eq("1"){
        println!("You Choose 777 Gamble");
        println!("Guess the number between 1 - 10");
        println!("Enter your guess");
        io::stdout().flush().unwrap();
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");
        if guess == String::from("2"){
            println!("You guessed correct");
        }else{
            println!("Your guess is wrong")
        }
    }else if trimmed_choice.eq("2"){
        println!("You choose Bingo");
    }
}

