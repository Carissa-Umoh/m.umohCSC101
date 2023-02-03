use std::io;

fn StudentCouncil(){
    for i in 0..15{ 
    fn main() {
        // to ask for the person's name
        let mut input1 = String::new();
        println!("What is your name?");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let mut name = input1.trim();

        // to ask for the person's email
        let mut input2 = String::new();
        println!("What is your email?");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let mut email = input2.trim();

        // to ask for the person's department
        let mut input3 = String::new();
        println!("What is your department?");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let mut dep = input3.trim();

        // to ask for the person's state of origin
        let mut input4 = String::new();
        println!("What is your state of origin?");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let mut state = input4.trim();
       
        // to ask if the person is a class rep
        let mut input5 = String::new();
        println!("\nAre you a class representative(y or n)?");
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let mut rep = input5.trim();
        

        // to ask if the person is in 100 level
        let mut input6 = String::new();
        println!("\nAre you in 100 level(y or n)?");
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let mut level = input6.trim();

        // to ask for the person's CGPA
        let mut input7 = String::new();
        println!("What is your cgpa?");
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let mut cgpa:f32 = input7.trim().parse().expect("Failed to read input");
        
        // calling the function to set range for CGPA3
        if rep == "y" && level == "y" && cgpa >= 4.0{
         println!("\nName: {}
        Department: {}
        State of Origin: {}
        Email: {}
        \nYou can vote!",name,dep,state,email);

        }
        else {
            println!("\nSorry,you are not eligible to vote");
        }
    
    }

}
