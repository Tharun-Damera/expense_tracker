use crate::expense::Expense;
use crate::user_cmd::UserCommand;

pub mod expense;
pub mod user_cmd;
pub mod utils;

fn main() {
    loop {
        println!("===========================");
        println!(" Rusty CLI Expense Tracker");
        println!("===========================");
        println!("1. Add Expense");
        println!("2. List Expenses");
        println!("3. Total Spending");
        println!("4. Filter by Category");
        println!("5. Exit");
        println!("---------------------------");
        println!("Enter your choice: ");
        let input: String = utils::read_input();

        let cmd = UserCommand::parse_input(&input);
        match cmd {
            UserCommand::Add => match Expense::add() {
                Ok(v) => println!("Expense added: {v:#?}"),
                Err(e) => {
                    println!("Please enter a valid amount and try again!");
                }
            },
            UserCommand::Exit => break,
            _ => println!("Feature is still under development"),
        }
    }
}
