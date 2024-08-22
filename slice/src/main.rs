fn main() {
    let a = String::from("Hello World");
    let c = &a[..5];
    //[0..5] = [..5]
    //[..] = full string
    //[5..till lenght of the string] = [5..]
    println!("{}",c);
}
