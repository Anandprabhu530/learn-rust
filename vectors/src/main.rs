use std::io;

fn main(){
    let mut a = vec![1,2,3,4];
    a.push(5);
    println!("Vector value {:?}",a);
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