fn main() {
    let ans = interpret(String::from("(al)G(al)()()G"));
    println!("{}",ans);
}

fn interpret(command: String) -> String {
    let mut ans = String::from("");
    let mut check = false;

    for word in command.chars(){
        if check==true && word==')' {
            ans.push_str("o");
        }
        if word == '('{
            check = true;
        }else if word==')'{
            check = false;
        }else{
            check = false;
            ans.push_str(&String::from(word));
        }
    }
    ans
}