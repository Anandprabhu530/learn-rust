fn main(){
    let (a,b,ans) = (1,2,String::from("Hello"));
    println!("The contents are {:?}",ans);

    //cannot do this bcz string cannot be added to int
    //println!("{}",ans+2);

    println!("The sum is {}", a+b);
}