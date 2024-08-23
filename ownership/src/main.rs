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

    let test1 = String::from("String to print");

    test_function(test1);
    
    //Cannot print test1 since it is moved bcz the function does not return.It dropped the value there
    //same like ownership transfer
    //but will work for primitives
    // println!("{}",test1);

    //can clone and spend else return

    //pass references 

    //added mut to line let c = mutable reference
    let mut a = String::from("Hello World");
    
    //make a immutable reference
    let b = &a;
    printer(b);
    

    //the printer function does not return but the value is never lost because we created a reference to it
    println!("Value of a is {}",b);

    //rust says there can only be only one mutable or infinite immutable references
    //to make a mutable reference the original variable should also be mutable
    let c = &mut a;

    //but here there is a mut and an immutable refernce bcz the compiler figured out that the b which has a &a is not used below

    //this will throw error on line 73 for creating a immutable reference
    // println!("Value of a is {}",b);

    //now let pass the mutable refernece to a function
    //receiving parameters should be like &mut (datatype)
    printer2(c);
 
    //cannot do this bcz already it checks a . At first b has a immutable reference of a followed by c with a mut reference
    //So When we try to access a now it has two references a mut and immutable
    //This is not permitted in rust
    //But it let us create a mut reference when there was a immutable reference bcz the immutable one ends it work before
    //declaration of mut reference.  The compiler figures it out
    // println!("{}",a);
    println!("{}",c);


    let mut tests = String::from("Test data");

    let testptr = &mut tests;
    printer2(testptr);

    println!("{}",tests)
}

fn printer(a: &String){
    println!("This is inside printer function {}",a)
}

fn printer2(a: &mut String){
    a.push_str(" from printer 2");


    //can deallocate manually with the *sign. But the rust takes care of the cleaning. But sometimes this will be useful
    //example
    // (*a).push_str(" from printer 2");
  
}

fn test_function(test1:String){
    println!("{}",test1);
    
}