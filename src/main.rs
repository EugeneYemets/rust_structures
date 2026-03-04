mod person; 
mod laptop; 

use crate::laptop::Laptop;
use crate::person::Person;

fn main() {
    let x = Laptop {
        model: "Asus".to_string(),
        year: 2024,
        temp: 43, 
        is_active: true,
        battery: 5,
        text: "Ви опанували раст! (на 2.2%)".to_string(), 
        owner: Person { 
            name: "Eugene".to_string(), 
            age: 16, 
            breakr: "ні".to_string(), 
            sell: "ні".to_string() 
        },  
    };

    println!("==============================================");
    println!("Laptop information: {} {}", x.model, x.year);
    println!("==============================================");
    
    x.is_active();
    x.print();
    x.temp_show();
    x.battery_show();
    x.print_charge_status();
    
    println!("=============================================="); 
    x.owner.breakr_status();
    x.owner.sell_status();
}