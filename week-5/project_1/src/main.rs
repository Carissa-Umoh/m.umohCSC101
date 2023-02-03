// Rust program to find the roots of a quadratic equation

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

 let mut d:f32 = 0.0;  // making b a squared number

    println!("Input a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Input b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Input c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let root1:f32 = -b + d.sqrt()/ (2.0*a);
    let root2:f32 = -b - d.sqrt()/ (2.0*a); 


    if a ==0.0 || b==0.0 || c==0.0
    {

    println!("Invalid Inputs");
    d = (b*b) - (4.0 *a*c) ;
}
    println!("root1 = {}",root1);
    println!("root2 = {}",root2);

    
    if d > 0.0
    {
    
    println!("There are two distinct roots");    
}
    if d == 0.0
    {
   
    println!("There is one real root");    
 }
     if d < 0.0
    {
        
    println!("There are no real roots");    
}

}
