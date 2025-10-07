use crate::expense::Expense;
use crate::expense_list::ExpenseList;
use crate::user_cmd::UserCommand;

pub mod expense;
pub mod expense_list;
pub mod user_cmd;
pub mod utils;

fn main() {
    let mut expense_list: Vec<Expense> = ExpenseList::load_expense_list();
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
            UserCommand::Add => match Expense::add(&mut expense_list) {
                Ok(_) => {
                    println!("Expense is added.")
                }
                Err(_) => {
                    println!("Please enter a valid amount and try again!");
                }
            },
            UserCommand::List => Expense::list(&expense_list),
            UserCommand::Total => {
                println!(
                    "Your total expenditure is {}",
                    Expense::total_expenditure(&expense_list)
                );
            }
            UserCommand::Filter => Expense::filter_by_category(&expense_list),
            UserCommand::Exit => {
                ExpenseList::save_expense_list(&expense_list)
                    .expect("Error while saving the expense list");
                println!("GoodBye!");
                break;
            }
            UserCommand::Invalid => continue,
        }
    }
}
