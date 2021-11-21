//use std::io;

//use std::io::Read;

#[derive(Debug)]
struct HelicopterData {
    fuel: u32,
    visibility: u32,
}
pub trait Flying {
    fn API();
    fn enter_data(&self);
    fn fly_helicopter();
}

impl Flying for HelicopterData {
    fn API() {
        todo!()
    }

    fn enter_data(&self) {
        println!("enter_data test");
    }

    fn fly_helicopter() {
        todo!()
    }
}


fn main() {

    let helicopter = HelicopterData {               //debug struct test
        fuel: 10,
        visibility: 75,
    };

    helicopter.enter_data();

    println!("Hello, world!");
    //println!("My helicopter test {:#?}",helicopter );   //debug line
    io_test();
}

fn io_test () {
    let mut var1 = String::new();
    println!("Enter the number of helicopters you want to fly");
    std::io::stdin().read_line(&mut var1).expect("Unable to read entered data");
    let copter_count: usize = var1.trim().parse().ok().expect("Program only processes numbers, enter a number");
    println!("There are {} helicopters.", copter_count);

    let mut copter_list: Vec<HelicopterData> = Vec::new();

    for i in 0..copter_count {
        var1.clear();
        println!("Enter the fuel level of helicopter");
        std::io::stdin().read_line(&mut var1).expect("Unable to read entered data");
        let fuel_level: u32 = var1.trim().parse().ok().expect("Program only processes numbers, enter a number");
        println!("Fuel level is {}.", fuel_level);

        var1.clear();
        println!("Enter the visibility for the helicopter");
        std::io::stdin().read_line(&mut var1).expect("Unable to read entered data");
        let visibility: u32 = var1.trim().parse().ok().expect("Program only processes numbers, enter a number");

        let temp_copter = HelicopterData {
            fuel: fuel_level,
            visibility,
        };

        copter_list.push(temp_copter);
        println!("Size test: {}", copter_list.len());
        println!("My helicopter test {:#?}",copter_list.get(i) );   //debug line
    }

}





