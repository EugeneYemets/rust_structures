use crate::person::Person;

#[derive(Debug)]
pub struct Laptop {
    pub model: String,
    pub year: u16,
    pub owner: Person,
    pub temp: u8,
    pub is_active: bool,
    pub battery: u8,
    pub text: String,
}

impl Laptop {
    pub fn is_active(&self) {
        println!("Ноутбук працює?: {}", self.is_active)
    }
    pub fn print(&self) {
        println!("{:?}", self.text)
    }
    pub fn temp_show(&self) {
        println!("Температура ноутбука: {}", self.temp)
    }
    pub fn battery_show(&self) {
        println!("Заряд ноутбука: {}", self.battery)
    }
    pub fn print_charge_status(&self) {
        let to_full = 100 - self.battery;
        println!("Зараз батарея: {}%, потрібно ще {}% до повного заряду.", self.battery, to_full);
    }
}