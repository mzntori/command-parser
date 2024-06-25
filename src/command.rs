use std::collections::{HashMap, HashSet};


/// Created from a string using a [`Parser`](crate::Parser).
///
/// For more detail look at [`Parser`](crate::Parser) documentation.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Command {
    pub prefix: char,
    pub option_prefix: char,
    pub name: String,
    pub arguments: Vec<String>,
    pub options: HashSet<String>,
    pub parameters: HashMap<String, String>
}

impl Command {
    pub fn new(
        prefix: char,
        option_prefix: char,
        name: String,
        arguments: Vec<String>,
        options: HashSet<String>,
        parameters: HashMap<String, String>
    ) -> Command {
        Command {
            prefix,
            option_prefix,
            name,
            arguments,
            options,
            parameters
        }
    }
}

