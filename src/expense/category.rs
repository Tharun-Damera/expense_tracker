use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Category {
    Food,
    Transport,
    Entertainment,
    Bills,
    Shopping,
    Miscellaneous,
}

impl Category {
    pub fn new() -> Self {
        println!("\nCategories:");
        println!("1. Food");
        println!("2. Transport");
        println!("3. Entertainment");
        println!("4. Bills");
        println!("5. Shopping");
        println!("6. Miscellaneous");

        println!("Select one of the above categories: ");
        let input = utils::read_input();

        match input.trim() {
            "1" => Category::Food,
            "2" => Category::Transport,
            "3" => Category::Entertainment,
            "4" => Category::Bills,
            "5" => Category::Shopping,
            _ => Category::Miscellaneous,
        }
    }
}
