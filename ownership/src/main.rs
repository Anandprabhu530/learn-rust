fn main(){

    //primitive values can be to another variable
    //Inside it uses clone since primitive or on stack - do not require much time
    let a = 1;
    let b = a;
    println!("Value of a is {}",a);
    println!("Value of b is {}",b);

    //While the heap allocated string cannot do that
    let sen1 = String::from("Hello World");
    let sen2 = sen1;

    //cannot do below bcz sen1 ownership transferred to sen2
    //Only one owner should be there
    // println!("Value of sen1 is {}", sen1);

    //sen2 is available
    println!("Value of sen2 after ownership transfer: {}", sen2);
    
    //if need sen1 to also exists and put it to sen2 you can clone it 
    let sen1 = String::from("Hello World");
    let sen2 = sen1.clone();
    println!("Value of sen1 is {}", sen1);
    println!("Value of sen2 after clonning: {}", sen2);
    {
        let sen3 = 10;
        println!("Value of sen3 inside scope {}",sen3)
    }

    //cannot access bcz the value is dropped when we exit the scope
    //println!("Value of sen3 is {}",sen3)
    let mut a = 10;
    println!("Value of A before scope {}",a);
    {
        a = 45;
        println!("Value of A inside scope {}",a);
        //even though we create a new a the old is present
        let mut a = 20;
        println!("Value of A new variable inside scope {}",a);
        a = 22;
        println!("Value of A new variable changed inside scope {}",a);
    }

    //value has been changed after going inside the scope -- Bcz We mutated it inside the scope
    println!("Value of A after scope {}",a);


}