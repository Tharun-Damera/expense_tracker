use std::io::{self, Read};
use std::path::Path;
use std::{fs, fs::File};

use crate::expense::Expense;
pub struct ExpenseList;

const JSON_FILE: &str = "expenses.json";

impl ExpenseList {
    pub fn load_expense_list() -> Vec<Expense> {
        let list: Vec<Expense> = Vec::new();
        if Path::new(JSON_FILE).exists() {
            if let Ok(mut file) = File::open(JSON_FILE) {
                let mut json = String::new();
                if let Ok(_v) = file.read_to_string(&mut json) {
                    if let Ok(vec) = serde_json::from_str(&json) {
                        let list: Vec<Expense> = vec;
                        return list;
                    }
                }
            }
        }
        list
    }

    pub fn save_expense_list(expenses: &Vec<Expense>) -> Result<(), io::Error> {
        let json = serde_json::to_string(expenses).expect("Failed to serialize");
        fs::write(JSON_FILE, json.as_bytes())?;
        Ok(())
    }
}
