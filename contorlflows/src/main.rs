fn main(){
    //if-else
    let a = String::from("Hello");
    let b:&str = "Hello";

    let c = if a=="Hello"{64}else{72};
    if a == b{
        println!("Equal")
    }else{
        println!("Not Equal");
    }
    println!("{}",c);

    //loop
    let mut counter = 1;
    let c = loop {
        counter+=1;
        if counter==10{
            break counter;
        }

    };
    println!("{}",c);

    //for loop from i = 1 to 10-1 == 9
    for num in 1..10{
        print!("{ }",num);
    }


}