//! # Command Syntax
//!
//! In any examples in this documentation `!` will be used as a prefix and `-` will be used as a option prefix.
//!
//! A command that this can parse could look like this:
//!
//! `!foo arg1 "long arg 2" -opt -opt -key1:val1 -key2:"long val2"`
//!
//! A command consists of 4 different parts:
//! - _name_: The name of the command is the first word after the prefix.
//! In the example above that's `foo`.
//! - _arguments_: Arguments are simple strings passed to the command.
//! They are either single words or strings with spaces enclosed by `"`.
//! In the example the two arguments are `arg1` and `long arg 2`.
//! - _options_: Options are a set of words.
//! They are prefixed with the `option_prefix`.
//! The only option in the example is `opt`.
//! - _parameters_: Parameters are key-value pairs.
//! They are prefixed with the `option_prefix` and seperated by `:`.
//! The value part of the pair can be a word or a string enclosed by `"`.
//! In the example above `key1`s value is `val1` and `key2`s value is `long val2`.
//!
//! # Escaping
//!
//! Since `"` is used to mark the borders of long arguments and values, it's not normally possible
//! to include them in the string of the argument.
//!
//! You can escape a long argument or value using \\:
//! - `\"`: produces `"`
//! - `\\`: produces `\`
//!
//! # Example
//!
//! ```
//! use std::collections::{HashMap, HashSet};
//! use command_parser::{Parser, Command};
//!
//! let p = Parser::new('!', '-');
//! let command_string = r##"!foo arg1 "long arg 2" -opt -opt -key1:val1 -key2:"long val2""##;
//! let command = p.parse(command_string).unwrap();
//!
//! assert_eq!(command.name, "foo");
//! assert_eq!(command.arguments[1], "long arg 2");
//! assert!(command.options.contains("opt"));
//! assert_eq!(command.parameters.get("key2"), Some(&"long val2".to_string()));
//! ```

mod command;
mod error;
mod parser;

pub use parser::*;
pub use command::*;
pub use error::*;
