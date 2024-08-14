fn main(){
    let a = 10;
    println!("A value after block is {a} ");
    let mut c:[i32; 3] = [1,2,34];
    let temp:i32 =23;
    c[1] = temp;
    let mut tuple = (1,2,"hello");
    tuple.1 = 3;
    let mut test:String = String::new();
    let a : String = String::from("Hello World");
    test = String::from("Testing");
    println!("A value after block is {a} ");
    let (data1,data2,data3) = tuple;

}