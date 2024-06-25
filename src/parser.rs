use std::collections::{HashMap, HashSet};
use crate::command::Command;
use crate::error::ParseError;
use crate::error::ParseError::{EscapeError, NameError, PrefixError};

#[derive(Debug, Copy, Clone)]
enum ParseState {
    Prefix,
    Name,
    Default,
    Argument,
    LongArgument,
    EscapeLongArg,
    Option,
    ParamConnector,
    ParamVal,
    ParamLongVal,
    EscapeLongParamVal,
}

/// Used to parse a [`Command`] from a string.
///
/// # Command Syntax
///
/// For more information about prefixes look at the fields of this struct.
/// In any examples in this documentation `!` will be used as a prefix and `-` will be used as a option prefix.
///
/// A command that this can parse could look like this:
///
/// `!foo arg1 "long arg 2" -opt -opt -key1:val1 -key2:"long val2"`
///
/// A command consists of 4 different parts:
/// - _name_: The name of the command is the first word after the prefix.
/// In the example above that's `foo`.
/// - _arguments_: Arguments are simple strings passed to the command.
/// They are either single words or strings with spaces enclosed by `"`.
/// In the example the two arguments are `arg1` and `long arg 2`.
/// - _options_: Options are a set of words.
/// They are prefixed with the `option_prefix`.
/// The only option in the example is `opt`.
/// - _parameters_: Parameters are key-value pairs.
/// They are prefixed with the `option_prefix` and seperated by `:`.
/// The value part of the pair can be a word or a string enclosed by `"`.
/// In the example above `key1`s value is `val1` and `key2`s value is `long val2`.
///
/// # Escaping
///
/// Since `"` is used to mark the borders of long arguments and values, it's not normally possible
/// to include them in the string of the argument.
///
/// You can escape a long argument or value using \\:
/// - `\"`: produces `"`
/// - `\\`: produces `\`
///
/// # Example
///
/// ```
/// use std::collections::{HashMap, HashSet};
/// use command_parser::{Parser, Command};
///
/// let p = Parser::new('!', '-');
/// let command_string = r##"!foo arg1 "long arg 2" -opt -opt -key1:val1 -key2:"long val2""##;
///
/// let command = Command {
///     prefix: '!',
///     option_prefix: '-',
///     name: "foo".to_string(),
///     arguments: vec!["arg1".to_string(), "long arg 2".to_string()],
///     options: HashSet::from(["opt".to_string()]),
///     parameters: HashMap::from([
///         ("key1".to_string(), "val1".to_string()),
///         ("key2".to_string(), "long val2".to_string())
///     ])
/// };
///
/// assert_eq!(p.parse(command_string).unwrap(), command);
/// ```
#[derive(Debug)]
pub struct Parser {
    /// Prefix of the command.
    ///
    /// `<prefix><name> ...`
    ///
    /// Should not be set to `' '` as most chats trim leading spaces.
    pub prefix: char,
    /// Prefix of options and parameters.
    ///
    /// `... <option_prefix><option> ... <option_prefix><param key>:<param value>`
    ///
    /// Should not be set to `' '` or `'"'` as it may not result in expected outcomes.
    pub option_prefix: char,
}

impl Parser {
    pub fn new(prefix: char, option_prefix: char) -> Parser {
        Parser {
            prefix,
            option_prefix,
        }
    }

    pub fn parse<'a>(&'_ self, raw: &'a str) -> Result<Command, ParseError> {
        let mut name = String::new();
        let mut arguments: Vec<String> = vec![];
        let mut options: HashSet<String> = HashSet::new();
        let mut parameters: HashMap<String, String> = HashMap::new();

        let mut state = ParseState::Prefix;
        let mut buffer = String::new();
        let mut key_buffer = String::new();

        for (cursor, c) in raw.chars().enumerate() {
            match state {
                ParseState::Prefix => {
                    match c {
                        x if x == self.prefix => {
                            state = ParseState::Name;
                        }
                        _ => {return Err(PrefixError(cursor, c))}
                    }
                }
                ParseState::Name => {
                    match c {
                        ' ' => {
                            if cursor == 1 {
                                return Err(NameError(cursor, c));
                            } else {
                                state = ParseState::Default;
                            }
                        }
                        _ => { name.push(c); }
                    }
                }
                ParseState::Argument => {
                    match c {
                        ' ' => {
                            arguments.push(buffer);
                            buffer = String::new();
                            state = ParseState::Default;
                        }
                        _ => {
                            buffer.push(c);
                        }
                    }
                }
                ParseState::LongArgument => {
                    match c {
                        '"' => {
                            arguments.push(buffer);
                            buffer = String::new();
                            state = ParseState::Default;
                        }
                        '\\' => {
                            state = ParseState::EscapeLongArg;
                        }
                        _ => {
                            buffer.push(c);
                        }
                    }
                }
                ParseState::EscapeLongArg => {
                    match c {
                        '"' | '\\' => {
                            state = ParseState::LongArgument;
                            buffer.push(c);
                        }
                        _ => {
                            return Err(EscapeError(cursor, c));
                        }
                    }
                }
                ParseState::Option => {
                    match c {
                        ' ' => {
                            options.insert(buffer);
                            buffer = String::new();
                            state = ParseState::Default;
                        }
                        ':' => {
                            key_buffer = buffer;
                            buffer = String::new();
                            state = ParseState::ParamConnector;
                        }
                        _ => {
                            buffer.push(c);
                        }
                    }
                }
                ParseState::ParamConnector => {
                    match c {
                        '"' => {
                            state = ParseState::ParamLongVal;
                        }
                        ' ' => {
                            parameters.insert(key_buffer, buffer);
                            key_buffer = String::new();
                            buffer = String::new();
                            state = ParseState::Default;
                        }
                        _ => {
                            state = ParseState::ParamVal;
                            buffer.push(c);
                        }
                    }
                }
                ParseState::ParamVal => {
                    match c {
                        ' ' => {
                            parameters.insert(key_buffer, buffer);
                            key_buffer = String::new();
                            buffer = String::new();
                            state = ParseState::Default;
                        }
                        _ => {
                            buffer.push(c);
                        }
                    }
                }
                ParseState::ParamLongVal => {
                    match c {
                        '"' => {
                            parameters.insert(key_buffer, buffer);
                            key_buffer = String::new();
                            buffer = String::new();
                            state = ParseState::Default;
                        }
                        '\\' => {
                            state = ParseState::EscapeLongParamVal;
                        }
                        _ => {
                            buffer.push(c);
                        }
                    }
                }
                ParseState::EscapeLongParamVal => {
                    match c {
                        '"' | '\\' => {
                            state = ParseState::ParamLongVal;
                            buffer.push(c);
                        }
                        _ => {
                            return Err(EscapeError(cursor, c));
                        }
                    }
                }
                ParseState::Default => {
                    match c {
                        ' ' => {}
                        '"' => {state = ParseState::LongArgument;}
                        x if x == self.option_prefix => {
                            state = ParseState::Option;
                        }
                        _ => {
                            state = ParseState::Argument;
                            buffer.push(c);
                        }
                    }
                }
            }
        }

        Ok(Command {
            prefix: self.prefix,
            option_prefix: self.option_prefix,
            name, arguments, options, parameters
        })
    }
}


#[cfg(test)]
pub mod tests {
    use std::time::{Duration, Instant};
    use super::*;

    #[test]
    fn parse_test() {
        let p = Parser::new('!', '-');
        let command_string = r##"!foo arg1 "long arg 2" -opt -opt -key1:val1 -key2:"long val2""##;

        let command = Command {
            prefix: '!',
            option_prefix: '-',
            name: "foo".to_string(),
            arguments: vec!["arg1".to_string(), "long arg 2".to_string()],
            options: HashSet::from(["opt".to_string()]),
            parameters: HashMap::from([
                ("key1".to_string(), "val1".to_string()),
                ("key2".to_string(), "long val2".to_string())
            ])
        };

        assert_eq!(p.parse(command_string).unwrap(), command);
    }

    #[test]
    fn time_test() {
        let p = Parser::new('!', '-');
        let command_string = r##"!foo arg1 "long arg 2" -opt -opt -key1:val1 -key2:"long val2""##;

        let now = Instant::now();

        for _ in 0..100000 {
            let _ = p.parse(command_string);
        }

        println!("{}", now.elapsed().as_micros());

        let p = Parser::new('!', '-');
        let command_string = r##"just a normal sentence"##;

        let now = Instant::now();

        for _ in 0..100000 {
            let _ = p.parse(command_string);
        }

        println!("{}", now.elapsed().as_micros());
    }

}
