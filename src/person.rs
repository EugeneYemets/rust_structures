#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub breakr: String,
    pub sell: String,
}

impl Person {
    pub fn breakr_status(&self) {
        println!(" Людина з імʼям {} та віком {} років зламала ноутбук? {}", self.name, self.age, self.breakr)
    }

    pub fn sell_status(&self) {
        println!(" Людина з імʼям {} та віком {} років продала ноутбук? {}", self.name, self.age, self.sell)
    }
}