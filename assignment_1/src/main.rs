use std::io::write;
use std::io;

fn code_7() {
    
let consulting = vec!["Analytics Consulting Services", "Customer Experience", "Cybersecurity, strategy, risk, compliance and resilience", "digital transformation", "Risk counting services", "Supply chain and operations", "Technology transformation"];
for a in consulting.iter(){
file.write_all("\nName: Aigbona Juliet"
    .as_bytes()).expect("not valid");
file.write_all("\nDepartment: Consulting"
    .as_bytes()).expect("not valid");
file.write_all("\nQualification: B.Sc"
    .as_bytes()).expect("not valid");
file.write_all(consulting.as_bytes()).expect("not valid");
     print!("{}",a);
}
let assurance = vec!["Audit services", "Climate changedand sustainability services", "Financial accounting advisory services", "Foresnsic and integrity services", "Private client audit experience", "Accounting link", "Assurance"];
for b in assurance.iter(){
    file.write_all("\nName: Akpevwe Iloka"
        .as_bytes()).expect("not valid");
    file.write_all("\nDepartment: Assurance"
       .as_bytes()).expect("not valid");
    file.write_all("\nQualification: HND"
        .as_bytes()).expect("not valid");
    file.write_all(assurance.as_bytes()).expect("write failed");
    print!("{}",b);
} 
fn code_9() {
let transactions = vec!["Corporate finance", "Divestments and carve-outs", "Sustainability and ESG Services", "M&A advisory", "M&A integration", "M&A Technology and tools", "M&A advanced analytics"];
for c in transactions.iter(){
    file.write_all("\nName: Maria Akinsola"
        .as_bytes()).expect("not valid");
    file.write_all("\nDepartment: Transactions and corporate finance"
        .as_bytes()).expect("not valid");
    file.write_all("\nQualification: M.Sc."
        .as_bytes()).expect("not valid");
    file.write_all(transactions.as_bytes()).expect("not valid");
    print!("{}",c);
}
let strategy = vec!["Strategy consulting", "Corporate and growth strategy", "Transaction strategy and execution", "Restructing and turnaround strategy", "Industry strategy","Digital business building", "Commercial strategy"];
for d in strategy.iter(){
    file.write_all("\nName: Ehis Ero"
        .as_bytes()).expect("not valid");
    file write_all("\nDepartment: Strategy"
        .as_bytes()).expect("not valid");
    file.write_all("\nQualification: M.Sc."
        .as_bytes()).expect("not valid");
    file write_all(strategy.as_bytes()).expect("not valid");
    print!("{}",d);
}
fn code_8() {
    let tax = vec!["Tax planning", "Tax function operations", "Tax policy and controversy", "Global trade", "Tax accounting", "Tax compliance", "Transaction tax"];
   for e in tax.iter(){
    file.write_all("\nName: Adamu Sagamu"
        .as_bytes()).expect("not valid");
    file.write_all("\nDepartment: Tax"
        .as_bytes()).expect("not valid");
    file.write_all("\nQualification: B.Sc."
        .as_bytes()).expect("not valid");
    file.write_all(tax.as_bytes()).expect("not valid");
    print!("{}",e);
   } 
 let people = vec!["Change management and experience", "HR transformation", "Integrated workforce mobility", "Learning and development consulting", "Recognition and reward advisory", "workforce analytics", "People and workforce"]
 for f in people.iter(){
    file.write_all("\nName: Gbenga Daniels"
        .as_bytes()).expect("not valid");
    file.write_all("\nDepartment: People and workforce"
        .as_bytes()).expect("not valid");
    file.write_all("\nQualification: HND"
        .as_bytes()).expect("not valid");
    file.write_all(people.as_bytes()).expect("not valid");
    print!("{}",f);
 }  

 fn main(){
    fn code_7(){};
    fn code_8(){};
    fn code_9(){};
    let mut file = std::fs::file::created{"aigbona_juliet.txt"}.expect("cannot create");
    file write_all("YOU ARE WELCOME TO ERNST & YOUNG GLOBAL LIMITED"
        .as_bytes()).expect("not valid");
    println!("\nData sent to file");

    let mut input1 = String:new{};
    println!("Type your department code");
    io::stdin().read_line(mut&input1).expect("not a valid input");
    let w = input1.trim().parse().expect("not a valid integer");
    let w ==7{
        code_7{};
     }else if w ==8{
        code_8{};
    }else if w ==9{
        code_9{};
    }else {
        println!("cannot be processed")
    }
    }
}
}   
}
