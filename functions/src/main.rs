fn main() {
    let a: i32 = 10;
    let c = new_function(a);
    println!("{c}");
    let (d ,test) = sting_returner();
    println!("{d} {test}"); 
}

//snake case is preffered for function names
//return type mention with ->
fn new_function(a:i32)->i32{
    println!("Inside New_function");
    let d = a * 4;
    //will be returned default by mentioning the variable only ---Without Semicolon---
    d
}

//multiple return 
fn sting_returner() -> (String,i32){
    let a = String::from("hello from string returner");
    return (a , 8);
}
