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
}