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

}

