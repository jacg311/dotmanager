use std::path::PathBuf;

pub enum RuleType {
    Directory,
    Pattern(String),
}

pub struct Rule {
    rule_type: RuleType,
    target_path: PathBuf,
}

impl Rule {
    fn parse(rule: String) -> Rule {
        let split: Vec<&str> = rule.split(':').collect();

        match split.len() {
            1 => Rule {
                rule_type: RuleType::Directory,
                target_path: PathBuf::from(split[0]),
            },
            2 => Rule {
                rule_type: RuleType::Pattern(String::from(split[1])),
                target_path: PathBuf::from(split[0]),
            },
            x => panic!("Cannot construct rule from {} elements.", x),
        }
    }

    fn matches(&self, file: PathBuf) -> bool {
        match &self.rule_type {
            RuleType::Directory => true,
            RuleType::Pattern(pattern) => {
                match pattern.find('*') {
                    Some(pos) => {
                        if pos = 0 {
                            file.ends_with(child)
                        }
                    },
                    None => file.starts_with(pattern),
                }
            }
        }
    }
}
