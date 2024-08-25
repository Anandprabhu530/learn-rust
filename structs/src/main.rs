struct Laptop{
    company:String,
    ram:i32,
    hdd:i32,
    os:Option<String>
}

//implementation
impl Laptop{
    fn printer(&self){
        println!("Inside implementation");
        println!("Laptop Brand:{}  Ram:{}gb, HDD:{}TB, OS:{:?}",self.company,self.ram,self.hdd,self.os);
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
}
