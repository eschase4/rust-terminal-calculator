use inquire::InquireError;

fn main() {
    println!("*******************CALCULATOR********************");
    calculate()
}

fn calculate() {
    let operations: Vec<&str> = vec!["Add", "Subtract", "Multiply", "Divide"];

    let ans: Result<&str, InquireError> = inquire::Select::new("What kind of operation would you like to use?", operations).prompt();
    let ans = ans.unwrap();

    println!("{}", ans);
}
