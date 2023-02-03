// rust program to check and validate staff employment validation

use std::io;
fn Pub_Service(){


    //If the staff is a public servant
    let mut input2 = String::new;
    println!("Are you a teacher?");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let mut public = input2.trim();

    //if the staff is an office administrator
    let mut input3 = String::new;
    println!("Are you an office administraton?");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let mut admin = input3.trim();

    //if the staff is  an academic staff
    let mut input4 = String::new;
    println!("Are you an academic staff?");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let mut aca = input4.trim();

    //if the staff is a lawyer
    let mut input5 String::new;
    println!("Are you a lawyer?");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let mut law = input5.trim();

    //if the staff is a teacher
    let mut input6 String:new;
    println!("Are you a teacher?");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let mut teach = input6.trim();

    


    //to ask how many years the staff has been working
    let mut input = String::new;
    println!("How many years of work experience do you have?");
    io::stdin().read_line(&mut input).expect("Failed to read input");


  // Array to represent the no of years for work experience
   let year = ["APS1-2","APS3-5","APS 5-8","EL1 8-10","EL2 10-13","ES14"];
   

  if admin && year == "APS1-2"{
  println!("intern");
  
 
  if aca && year == "APS1-2"{
    println!("Not eligible for promotion");
  }
  
  if law && year =="APS1-2"{
    println!("paralegal");
  }
}
