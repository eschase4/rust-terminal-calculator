use inquire::InquireError;
use std::io;
fn main() {
    println!("**********************CALCULATOR***********************");
    calculate();
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
    } else if ans == "Subtract" {
        subtract(num1, num2);
    } else if ans == "Multiply" {
        multiply(num1, num2);
    } else if ans == "Divide" {
        divide(num1, num2);
    }

}

fn recalculate(result: f64){
    let operations: Vec<&str> = vec!["Add", "Subtract", "Multiply", "Divide"];

    let ans: Result<&str, InquireError> = inquire::Select::new("What kind of operation would you like to use?", operations).prompt();
    let ans = ans.unwrap().trim();

    let mut num = String::new();

    println!("What number would you like to {}?", ans);

    io::stdin().read_line(&mut num).expect("Failed to read line");


    let num: f64 = num.trim().parse().expect("Please enter a number");
    
    if ans == "Add" {
        add(result, num);
    } else if ans == "Subtract" {
        subtract(result, num);
    } else if ans == "Multiply" {
        multiply(result, num);
    } else if ans == "Divide" {
        divide(result, num);
    }
    return
}

fn questions(result: f64){
    //Do another operation on the current result
    let options: Vec<&str> = vec!["Yes", "No"];
    let ans: Result<&str, InquireError> = inquire::Select::new("Would you like to do another operation on this result?", options).prompt();
    let ans = ans.unwrap().trim();
    
    if ans == "Yes" {
        recalculate(result);
        return
    }
    
    // Reset Calculator
    let options: Vec<&str> = vec!["Yes", "No"];
    let ans: Result<&str, InquireError> = inquire::Select::new("Would you like to do a new operation?", options).prompt();
    let ans = ans.unwrap().trim();
    if ans == "Yes" {
        calculate();
    } else {
        println!("*******************************************************");
        return;
    }
    return
}

fn add(num1: f64, num2: f64){
    let result = num1 + num2;
    
    println!("The result of {} + {} is {}", num1, num2, result);
    questions(result);
}

fn subtract(num1: f64, num2: f64){
    let result: f64 = num1 - num2;
    println!("The result of {} - {} is {}", num1, num2, result);
    questions(result);
}

fn multiply(num1: f64, num2: f64){
    let result: f64 = num1 * num2;
    println!("The result of {} x {} is {}", num1, num2, result);
    questions(result);
}

fn divide(num1: f64, num2: f64){
    let result: f64 = num1 / num2;
    println!("The result of {} ÷ {} is {}", num1, num2, result);
    questions(result);
}