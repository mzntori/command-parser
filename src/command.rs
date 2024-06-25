use std::collections::HashMap;

// <prefix><name> <arguments> -<option> -<parameter_key>:<parameter_value> -<parameter_key>:"<parameter_value>"
// !ping mzntori "test test" -yes -t -mhm:true -test:"false stuff"

#[derive(Debug)]
pub struct Command {
    prefix: String,
    name: String,
    arguments: Vec<String>,
    options: Vec<String>,
    parameters: HashMap<String, String>
}