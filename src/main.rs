use inquire::InquireError;
use std::io;
fn main() {
    println!("*******************CALCULATOR********************");
    calculate()
}

fn calculate() {
    let operations: Vec<&str> = vec!["Add", "Subtract", "Multiply", "Divide"];

    let ans: Result<&str, InquireError> = inquire::Select::new("What kind of operation would you like to use?", operations).prompt();
    let ans = ans.unwrap().trim();

    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("What two numbers would you like to {}?", ans);
    println!("Number 1:");

    io::stdin().read_line(&mut num1).expect("failed to read line");

    println!("Number 2:");
    io::stdin().read_line(&mut num2).expect("failed to read line");

    let num1: f64 = num1.trim().parse().expect("Please enter a number");
    let num2: f64 = num2.trim().parse().expect("Please enter a number");

    if ans == "Add" {
        add(num1, num2);
    }

}

fn recalculate(){
    
}

fn add(num1: f64, num2: f64){
    let result = num1 + num2;
    let options: Vec<&str> = vec!["Yes", "No"];

    println!("The result of {} + {} is {}", num1, num2, result);

    let ans: Result<&str, InquireError> = inquire::Select::new("Would you like to do another operation on this result?", options).prompt();
}