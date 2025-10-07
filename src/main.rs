
use user_cmd::UserCommand;

pub mod utils;
pub mod user_cmd;

fn main() {
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
    dbg!(cmd);
}
