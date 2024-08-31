use std::io;

fn main() {
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("failed to read from stdin");
    let trimmed_username = username.trim();
    println!("Welcome {}!! Let's Gamble",trimmed_username)
}

