#[derive(Debug)]
pub enum UserCommand {
    Add,
    List,
    Total,
    Filter,
    Exit,
    Invalid,
}

impl UserCommand {
    pub fn parse_input(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "1" => UserCommand::Add,
            "2" => UserCommand::List,
            "3" => UserCommand::Total,
            "4" => UserCommand::Filter,
            "5" => UserCommand::Exit,
            _ => UserCommand::Invalid,
        }
    }
}
