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
    fn fly_helicopter(copters: &mut Vec<HelicopterData>);
}

impl Flying for HelicopterData {
    fn API(&self) {
        println!("In API");
        //add copter list here and see if return works
        let mut _copter_list:Vec<HelicopterData> = Self::enter_data();
        Self::fly_helicopter(&mut _copter_list);
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

    fn fly_helicopter(copters: &mut Vec<HelicopterData>) {

        while copters.len() > 0 {
            let mut i = 0;
            while i < copters.len() {
                if copters[i].visibility < 60 {
                    println!("Not safe to fly");
                    copters.remove(i);
                } else if copters[i].fuel >= 10 {
                    println!("Flying copter {}...now with fuel {}%",i+1, copters[i].fuel);
                    copters[i].fuel = copters[i].fuel - 2;
                } else {
                    println!("Low fuel {}% landing now", copters[i].fuel);
                    copters.remove(i);
                }
                i += 1;
            }
        }
        /*
        //while copters.len() > 0 {
            for copter in copters.iter_mut() {
                if copter.visibility < 60 {
                    println!("Not safe to fly");
                } else if copter.fuel >= 10 {
                    println!("Flying...now with fuel {}%", copter.fuel);
                    copter.fuel - 2;
                } else {
                    println!("Low fuel {}% landing now", copter.fuel);
                    //copters.pop();
                }
            }
        //}*/
    }
}


fn main() {
    println!("Hello, world!");
    let helicopter = HelicopterData {               //debug struct test
        fuel: 10,
        visibility: 75,
    };


    helicopter.API();



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





