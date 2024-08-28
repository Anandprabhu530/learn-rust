use std::f32::consts::PI;

struct Laptop{
    company:String,
    ram:i32,
    hdd:i32,
    os:Option<String>
}
//Sample for rust-docs
//add the below to print structs
#[derive(Debug)]
struct sample{
    radius:i32
}

impl sample {
    fn area(&self)->f32{
        (self.radius*self.radius) as f32 *3.14
    }
}
//implementation
impl Laptop{
    fn printer(&self){
        println!("Inside implementation");
        println!("Laptop Brand:{}  Ram:{}gb, HDD:{}TB, OS:{:?}",self.company,self.ram,self.hdd,self.os);
    }
}

struct test{
    number:i32
}

impl test {
    fn alter(&mut self,value:i32){
        self.number = value;
    }
}
fn main() {
    let mut a = Laptop{
        company:String::from("Dell"),
        ram:16,
        hdd:1,
        os:Some(String::from("Linux"))
    };
    println!("Laptop Brand:{}  Ram:{}gb, HDD:{}TB, OS:{:?}",a.company,a.ram,a.hdd,a.os);

    println!("Upgrade Laptop");
    a.ram = 32;
    a.hdd = 2;
    if let Some(os) = &a.os {
        println!("{:?}'s bag has {}!", a.os, os)
    } else {
        println!("{}'s bag is empty!", a.company)
    }
    // println!("Laptop Brand:{}  Ram:{}gb, HDD:{}TB",a.company,a.ram,a.hdd);
    a.printer();

    let circle1 = sample{
        radius:5
    };
    let ans = area(&circle1);
    println!("The area of the circle is {}",ans);
    println!("The actual circle object is {:?}",circle1);
    
    //below will be implemented with methods for the same
    let ans1 = circle1.area();
    println!("The area of the circle with imp is {}",ans1);
    let mut sample = test{
        number:20
    };
    sample.alter(32);
    println!("{:?}",sample.number)

}

fn area(circle:&sample)->f32{
    (circle.radius*circle.radius) as f32*3.14
}