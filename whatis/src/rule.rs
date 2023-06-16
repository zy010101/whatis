use std::{fs::{File, self}, io::BufReader};
use anyhow::{Result, Ok};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::settings::CONFIG;

pub static RULES: Lazy<Rules> = Lazy::new(|| {
    let rules = compile_rule().unwrap();
    rules
});

#[derive(Debug, Deserialize, PartialEq)]
struct Rule {
    name: String,
    description: String,
    enabled: bool,
    keywords: Option<Vec<String>>,
    rule: String,
    exceptions: Option<Vec<String>>,
    validation: Option<String>,

    #[serde(default = "default_keyword_max_distance")]
    keyword_max_distance: Option<u64>,
    // tags: Option<Vec<String>>,
    // example: Option<Vec<String>>
}

#[derive(Debug)]
pub struct CompileRule {
    pub name: String,
    pub description: String,
    pub keywords: Option<Vec<Regex>>,
    pub rule: Regex,
    pub exceptions: Option<Vec<Regex>>,
    pub validation: Option<String>,
    pub keyword_max_distance: u64,
    // pub tags: Option<Vec<String>>,
    // pub example: Option<Vec<String>>
}

pub struct Rules {
    pub rules: Vec<CompileRule>
}

impl CompileRule {
    fn new(name: String, description: String, keywords: Option<Vec<Regex>>, rule: Regex,
        exceptions: Option<Vec<Regex>>, validation: Option<String>, keyword_max_distance: u64) -> Self
    {
        Self { name, description, keywords, rule, exceptions, validation, keyword_max_distance }
    }
}

fn default_keyword_max_distance() -> Option<u64> {
    Some(CONFIG.rule.keyword_max_distance_default)
}

fn read_rules() -> Result<Vec<Rule>> {
    let dir_path = "rules";
    let dir_entries = fs::read_dir(dir_path)?;
    let mut rules = Vec::new();

    for entry in dir_entries {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            let rule : Rule = serde_yaml::from_reader(reader)?;
            rules.push(rule);
        }
    }

    Ok(rules)
}

fn compile_rule() -> Result<Rules> {
    let rules = read_rules()?;
    let mut compile_rules = Vec::new();

    for rule in rules {
        let name = rule.name;
        let description = rule.description;

        let keywords = if CONFIG.rule.enable_keywords {
            match rule.keywords {
                Some(keywords) => {
                    let mut kws = Vec::new();
                    for keyword in keywords {
                        kws.push(Regex::new(&keyword)?);
                    }
                    Some(kws)
                },
                None => None,
            }
        }else {
            None
        };


        let c_rule = Regex::new(&rule.rule)?;
        let exceptions = match rule.exceptions {
            Some(exceptions) => {
                let mut excepts = Vec::new();
                for exception in exceptions {
                    excepts.push(Regex::new(&exception)?);
                }
                Some(excepts)
            },
            None => None,
        };

        let validation = rule.validation;
        let keyword_max_distance = match rule.keyword_max_distance {
            Some(keyword_max_distance) => {
                keyword_max_distance
            },
            None => {
                CONFIG.rule.keyword_max_distance_default
            }
        };
        
        let rule = c_rule;
        let compile_rule = CompileRule::new(name, description, keywords, rule, exceptions, validation, keyword_max_distance);
        compile_rules.push(compile_rule);
    }

    Ok(Rules { rules: compile_rules })
}


#[cfg(test)]
mod tests{
    use std::{fs::File, io::BufReader};

    use super::{Rule, read_rules};

    #[test]
    fn read_yaml() {
        let file = File::open("demo.yaml").unwrap();
        let reader = BufReader::new(file);
        let rule: Rule = serde_yaml::from_reader(reader).unwrap();
        assert_eq!(rule.description, "Chinese Mainland ID number，excluding Hong Kong, Macao, and Taiwan".to_string());
        assert_eq!(rule.name, "Chinese Mainland ID number".to_string());
        assert_eq!(rule.exceptions.unwrap(), vec!["His employee ID is 372522197003231000"]);
        assert_eq!(rule.keyword_max_distance, Some(30));
        assert_eq!(rule.keywords.unwrap(), vec!["(?i)id", "身份证", "证件", "证明"]);
        assert_eq!(rule.rule, r"(?i)(1[1-5]|2[1-3]|3[1-7]|4[1-6]|5[0-4]|6[1-5]|71|8[1-3])(0[1-9]|[1-6][0-9]|70)\d{2}(19|20)\d{2}(0[1-9]|1[0-2])(0[1-9]|[1-2][0-9]|3[0-1])\d{3}[0-9x]");
        assert_eq!(rule.validation.unwrap(), "chinese_mainland_id_number".to_string());
    }

    #[test]
    fn rules() {
        let res = read_rules().unwrap();
        for r in res {
            println!("{}", r.name);
        }
    }

}

