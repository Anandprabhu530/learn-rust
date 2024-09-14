use std::io;

fn main(){
    let mut a = vec![1,2,3,4];
    a.push(5);
    println!("Vector value {:?}",a);
    let second = &a[2];
    println!("The 2nd element is {second}");
    let also_second = a.get(2);

    match also_second {
        Some(number)=>println!("The 2nd element is {number} -- done with get() and pattern matching"),
        None => ()
    }

    let mut input_vec = Vec::new();
    let mut input_enter = String::new();
    io::stdin()
        .read_line(&mut input_enter)
        .expect("There was an error");
    let input_enter_push = input_enter.trim();
    input_vec.push(input_enter_push);

    //cannot do this since string is pushed
    // input_vec.push(1);
    println!("Vector value {:?}",input_vec);

}