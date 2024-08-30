#[derive(Debug)]
enum Category{
    Action(String),
    Horror(String),
    Adventure(String),
}

#[derive(Debug)]
struct MovieN{
    rating:Category,
    price:i32
}

impl MovieN{
    fn printer(&self){
        println!("Inside impl printer");
        println!("{:?}",self.rating);
    }
}

fn main(){

    //You can also do with structs
    let movie1 = MovieN{
        rating:Category::Action(String::from("Avengers")),
        price:130
    };

    movie1.printer();

    //you can also do with enums
    let movie2 = Category::Horror(String::from("Friday 13th"));
    let movie3 = Category::Adventure(String::from("Tomb Raider"));

    println!("Movie 1: {:#?}, Movie 2: {:?}, Movie 3: {:?}",movie1,movie2,movie3);

    let sample1 = Some(String::from("Hello World"));
    let sample2 = Some(32);
    let temp = 2;
    let sample3:Option<String> = None;
    
    //the below is not possible bcz i32 and Option<i32> are different types
    //we use match for it
    // let sum = sample2+temp;

    //match the pattern
    fn add_number(x:Option<i32>,)->i32{
        match x{
            None=>0,
            Some(a) => a,
        }
    }

    let ans = add_number(sample2);

    //Can also return the option with some itself
    fn add_number2(x:Option<i32>,b:i32)->Option<i32>{
        match x{
            None=>None,
            Some(a) => Some(a+b),
        }
    }

    let ans2 = add_number2(sample2,temp);
    println!("The ans is {:?}", ans2);

    let sum = ans + temp;
    println!("The ans is {:?}", sum);
    
   
}

