use std::path::PathBuf;

use glob::{MatchOptions, Pattern};

pub enum Rule {
    Directory(PathBuf),        // global target
    Pattern(Pattern, PathBuf), // symlink files that match the pattern to the target
    Negative(Pattern),         // do not symlink files that match the pattern
}

impl Rule {
    pub fn parse(mut rule: String) -> Option<Rule> {
        if rule.trim().is_empty() {
            return None
        }
        if rule.starts_with('!') {
            rule.remove(0);
            return Some(Rule::Negative(Pattern::new(rule.as_str()).unwrap()));
        }
        let split: Vec<&str> = rule.split(':').collect();

        match split.len() {
            1 => Some(Rule::Directory(PathBuf::from(split[0]))),
            2 => Some(Rule::Pattern(
                Pattern::new(split[1]).unwrap(),
                PathBuf::from(split[0]),
            )),
            _ => {
                eprintln!("Cannot construct a rule from this string: {rule}");
                None
            },
        }
    }

    fn apply(&self, file: PathBuf) {}
}
