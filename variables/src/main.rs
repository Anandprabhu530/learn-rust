const DATA: &str = "Hello for const";
static mut TEST_FOR_STATIC:  &str = "Hello for Static";
fn main(){
    let a = 10;
    println!("A value after block is {a} ");
    let mut c:[i32; 3] = [1,2,34];
    let temp:i32 =23;
    c[1] = temp;
    let mut tuple = (1,2,"hello");
    tuple.1 = 3;
    let mut test:String = String::new();
    let mut a : String = String::from("Hello World");
    test = String::from("Testing");
    a = test;
    println!("A value after block is {a} ");
    let (_data1,_data2,_data3) = tuple;
    println!("{DATA}");
    //can mutate static only in unsafe mode
    unsafe { println!("{TEST_FOR_STATIC}")}
}