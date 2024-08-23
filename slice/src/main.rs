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

}


fn adder(text: &mut str)->&str{
    &text[0..5]
}

fn adder2(text: &str)->&str{
    &text[0..5]
}