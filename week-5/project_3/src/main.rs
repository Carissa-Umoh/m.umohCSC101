// Rust program to display food menu

use std:io;

fn main() 
{
    println!("Welcome to Favour's kitchen!");
    println!("Click on enter to display menu");

let food = ["Poundo Yam/Edikinko soup", "Fried rice & Chicken", "Amala & Ewedu Soup", "Eba & Egusi Soup", "White Rice & Stew"];
let price = [3200.00, 3000.00, 2500.00, 2000.00, 2500.00];

   println!("If you want Poundo Yam/Edikinko soup\ntype 1.0");
   println!("If you want Fried rice & Chicken\ntype 2.0");
   println!("If you want Amala & Ewedu Soup\ntype 3.0");
   println!("If you Eba & Egusi Soup\ntype 4.0");
   println!("If you White Rice & Stew\ntype 5.0");

    let mut input1 = String::new();
    let mut input2 = String::new();
  
   println!("Place your order");
   io:stdin().read_line(&mut input1).expect("Not a valid string");
   let 
