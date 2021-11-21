//use std::io;

//use std::io::Read;

#[derive(Debug)]
pub struct HelicopterData {
    fuel: u32,
    visibility: u32,
}
pub trait Flying {
    fn API(&self);
    //fn enter_data();
    fn enter_data()->Vec<HelicopterData>;
    fn fly_helicopter() ->i32;
}

impl Flying for HelicopterData {
    fn API(&self) {
        println!("In API");
        //add copter list here and see if return works
        let mut _copter_list:Vec<HelicopterData> = Self::enter_data();
        println!("API calling fly is {}", Self::fly_helicopter());
        //Self::enter_data();

        for copter in _copter_list {
            println!("In API my copter is:  {:#?}", copter );   //debug line
        }

    }

    fn enter_data() -> Vec<HelicopterData> {
        println!("enter_data test");

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
            //println!("Size test: {}", copter_list.len());
            //println!("My helicopter test {:#?}",copter_list.get(i) );   //debug line
        }
       copter_list
    }

    fn fly_helicopter() ->i32 {
        5
    }
}


fn main() {

    let helicopter = HelicopterData {               //debug struct test
        fuel: 10,
        visibility: 75,
    };

    helicopter.API();
    //println!("Test  Enter_date2: ");


    println!("Hello, world!");
    //println!("My helicopter test {:#?}",helicopter );   //debug line
    //io_test();
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





