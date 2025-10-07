use std::num::ParseFloatError;

use chrono::{DateTime, Local};

use crate::expense::category::Category;
use crate::utils;

pub mod category;

#[derive(Debug)]
pub struct Expense {
    id: u32,
    amount: f32,
    category: Category,
    date: DateTime<Local>,
    note: String,
}

impl Expense {
    fn new(amount: f32, category: Category, note: String) -> Self {
        Expense {
            id: 1,
            amount: amount,
            category: category,
            date: Local::now(),
            note: note,
        }
    }

    pub fn add() -> Result<Self, ParseFloatError> {
        println!("Enter the expense amount: ");
        let amount: f32 = utils::read_input().trim().parse()?;
        let category: Category = Category::new();
        println!("Enter a note describing the expense: ");
        let note: String = utils::read_input().trim().to_string();

        Ok(Self::new(amount, category, note))
    }

    pub fn list(expenses: &Vec<Expense>) {
        if expenses.is_empty() {
            println!("No expenses recorded yet.")
        } else {
            println!("These are your current expenses");
            for expense in expenses {
                println!("{:?}", expense);
            }
        }
    }

    pub fn total_expenditure(expenses: &Vec<Expense>) -> f32 {
        expenses.iter().map(|x| x.amount).sum()
    }
}
