use std::fmt::{self, Display};
use std::num::ParseFloatError;

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::expense::category::Category;
use crate::utils;

pub mod category;

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    id: u32,
    amount: f32,
    category: Category,
    date: String,
    note: String,
}

impl Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} • {} • {} • {}",
            self.id, self.date, self.category, self.amount, self.note
        )
    }
}

impl Expense {
    fn new(id: u32, amount: f32, category: Category, note: String) -> Self {
        Expense {
            id,
            amount,
            category,
            date: Local::now().to_rfc3339(),
            note,
        }
    }

    pub fn add(expenses: &mut Vec<Expense>) -> Result<(), ParseFloatError> {
        println!("Enter the expense amount: ");
        let amount: f32 = utils::read_input().trim().parse()?;
        let category: Category = Category::new();
        println!("Enter a note describing the expense: ");
        let note: String = utils::read_input().trim().to_string();

        let id: u32 = expenses.last().map(|x| x.id + 1).unwrap_or(1);
        expenses.push(Self::new(id, amount, category, note));
        Ok(())
    }

    pub fn list(expenses: &Vec<Expense>) {
        if expenses.is_empty() {
            println!("No expenses recorded yet.")
        } else {
            println!("These are your current expenses");
            println!("[Id] Date • Category • Amount • Note");
            for expense in expenses {
                println!("{}", expense);
            }
        }
    }

    pub fn total_expenditure(expenses: &Vec<Expense>) -> f32 {
        expenses.iter().map(|x| x.amount).sum()
    }

    pub fn filter_by_category(expenses: &Vec<Expense>) {
        let selected_category = Category::new();
        let category_expenses: Vec<&Expense> = expenses
            .iter()
            .filter(|x| x.category == selected_category)
            .collect();
        if category_expenses.is_empty() {
            println!("No expenses recorded in the selected category");
        } else {
            for expense in category_expenses {
                println!("{:?}", expense);
            }
        }
    }
}
