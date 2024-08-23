fn main() {
    let a = String::from("Hello World");
    let c = &a[..5];
    //[0..5] = [..5]
    //[..] = full string
    //[5..till lenght of the string] = [5..]
    println!("{}",c);
    let mut a = String::from("Familyguy is really good");
    //this works because the string are chnaged automatically to String slices
    //But the slice will not change to String
    println!("{}",a);
    
    let ans = adder(&mut a);
    println!("{}",ans);
    
    let b = "Heloisnotgood";
    //cannot create a mut reference to b bcz b is of type &str
    //Will learn later about it
    let ans2 = adder2(&b);
    println!("{}",ans2);

    //move to strings
    let a =  "hello World";
    //to_sring() converts &str to String type
    let _b = a.to_string();
    //format macro converts &str to String type format is slow and costly sometimes
    let _c = format!("{}",a);
    //to_owned() converts &str to String type
    let _d =  a.to_owned();

    //pushing to strings
    //first make it mut
    let mut a = String::from("Hello, ");
    a.push_str("World");
    println!("{}",a);
    //can also use concat
    let mut b = "Hello, ";
    println!("{}",b);

    //can concat like these or like below
    let c = concat!("Hello","World");
    //another way to concatenate
    b = "Yest";
    let temp = String::from("Test temp value");

    //cannot pass temp itself pass a ref to it like a string literal
    let d = [b,&temp].concat();

    println!("{}",d);
    println!("{}",c);



}


fn adder(text: &mut str)->&str{
    &text[0..5]
}

fn adder2(text: &str)->&str{
    &text[0..5]
}