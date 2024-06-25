use regex::Regex;

// <prefix><name> <arguments> -<option> -<parameter_key>:<parameter_value> -<parameter_key>:"<parameter_value>"
// !ping mzntori "test test" -yes -t -mhm:true -test:"false stuff"

#[derive(Debug)]
pub struct Parser {
    prefix: String,
    name_regex: Regex,
    option_prefix: String,
    option_regex: Regex,
}

impl Parser {
    pub fn new(prefix: String, option_prefix: String) -> Parser {
        Parser {
            name_regex: Parser::name_regex_from_prefix(prefix.as_str()),
            prefix,
            option_regex: Self::option_regex_from_option_prefix(option_prefix.as_str()),
            option_prefix,
        }
    }

    /// Checks whether a hay is a valid command.
    /// If this is false, all parse methods will return [`None`] or an empty [`Vec`] by default.
    ///
    /// A valid command is defined by starting with the prefix returned by [`self.get_prefix`] as the first characters.
    /// Followed by a valid command name.
    /// A valid command name is any ASCII with at least 1 character.
    /// A command name also can't contain a space as the second word will be counted as an argument.
    /// This essentially checks if [`self.parse_name`] returns a valid name but with faster runtime.
    ///
    /// # Examples
    /// Using `!` as a prefix:
    ///
    /// Valid commands:
    /// - `!foo`
    /// - `!foo bar` (`bar` is an argument and is not part of the name here)
    /// - `!foo -bar`
    /// - `!foo -bar:test`
    ///
    /// Invalid commands:
    /// - `! do`
    /// - `a! do`
    /// - `\n! do`
    pub fn valid(&self, hay: &str) -> bool {
        self.name_regex.is_match(hay)
    }

    /// Returns the command name of a given haystack.
    /// If no command name could be found returns [`None`].
    pub fn parse_name<'a>(&'_ self, hay: &'a str) -> Option<&'a str> {
        if let Some(captures) = self.name_regex.captures(hay) {
            captures.get(1).map(|v| { v.as_str() })
        } else {
            None
        }
    }

    pub fn parse_options<'a>(&'_ self, hay: &'a str) -> Vec<&'a str> {
        // Check if valid.
        let mut options: Vec<&'a str> = vec![];

        if !self.valid(hay) {
            return options;
        }

        for capture in self.option_regex.find_iter(hay) {
            options.push(&capture.as_str().trim_end()[self.option_prefix.len()..]);
        }

        options
    }

    /// Sets the prefix to a given string.
    /// Any ASCII string is valid.
    pub fn set_prefix(&mut self, prefix: String) {
        self.name_regex = Parser::name_regex_from_prefix(prefix.as_str());
        self.prefix = prefix;
    }

    /// Returns a reference to the prefix.
    pub fn get_prefix(&self) -> &str {
        self.prefix.as_str()
    }

    fn name_regex_from_prefix(prefix: &str) -> Regex {
        Regex::new(format!("\\A{}([^ ]+)", regex::escape(prefix)).as_str()).unwrap()
    }

    // "-([^ :]*)[^ ]*"
    fn option_regex_from_option_prefix(option_prefix: &str) -> Regex {
        Regex::new(format!(" {}([^ :]+)(?: |$)", regex::escape(option_prefix)).as_str()).unwrap()
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test() {
        // regex for command name (using ! as a prefix)
        // let name_re = Regex::new(r"^!(.\w+)").unwrap();
        // let hay = "!ping test";

        let parser = Parser::new("!".to_string(), "opt:".to_string());
        let hay = r"!2 'test test2' 'test test1' opt:yes -tt -mhm:true -opt:'false stuff' -t";
        dbg!(parser.valid(hay));
        dbg!(parser.get_prefix());
        dbg!(parser.parse_name(hay));
        dbg!(parser.parse_options(hay));
    }
}
